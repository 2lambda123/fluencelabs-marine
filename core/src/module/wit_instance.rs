/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::wit_prelude::*;
use super::marine_module::MModule;
use super::IRecordType;
use crate::MResult;

use marine_wasm_backend_traits::WasmBackend;
//use marine_wasm_backend_traits::Module;
use marine_wasm_backend_traits::Instance;
use marine_wasm_backend_traits::ImportObject;
use marine_wasm_backend_traits::Exports;
use marine_wasm_backend_traits::Memory as WBMemory;

use marine_it_interfaces::MITInterfaces;
use marine_it_interfaces::ITAstType;

use wasmer_it::interpreter::wasm;
use wasmer_it::interpreter::wasm::structures::{LocalImportIndex, Memory, TypedIndex};
//use wasmer_core::Instance as WasmerInstance;

use std::collections::HashMap;
use std::rc::Rc;

pub type MRecordTypes = HashMap<u64, Rc<IRecordType>>;

/// Contains all import and export functions that could be called from IT context by call-core.
#[derive(Clone)]
pub(super) struct ITInstance<WB: WasmBackend> {
    /// IT functions indexed by id.
    funcs: HashMap<usize, WITFunction<WB>>,

    /// IT memories.
    memories: Vec<<WB as WasmBackend>::WITMemory>,

    /// All record types that instance contains.
    record_types_by_id: MRecordTypes,
}

impl<WB: WasmBackend> ITInstance<WB> {
    pub(super) fn new(
        wasmer_instance: &<WB as WasmBackend>::Instance,
        module_name: &str,
        wit: &MITInterfaces<'_>,
        modules: &HashMap<String, MModule<WB>>,
    ) -> MResult<Self> {
        let mut exports = Self::extract_raw_exports(wasmer_instance, wit)?;
        let imports = Self::extract_imports(module_name, modules, wit, exports.len())?;
        let memories = Self::extract_memories(wasmer_instance);

        exports.extend(imports);
        let funcs = exports;

        let record_types_by_id = Self::extract_record_types(wit);

        Ok(Self {
            funcs,
            memories,
            record_types_by_id,
        })
    }

    fn extract_raw_exports(
        wasmer_instance: &<WB as WasmBackend>::Instance,
        it: &MITInterfaces<'_>,
    ) -> MResult<HashMap<usize, WITFunction<WB>>> {
        let module_exports = &wasmer_instance.exports();

        it.exports()
            .enumerate()
            .map(|(export_id, export)| {
                let export_func = module_exports.get_dyn_func(export.name)?;
                Ok((
                    export_id,
                    WITFunction::from_export(export_func, export.name.to_string())?,
                ))
            })
            .collect()
    }

    /// Extracts only those imports that don't have implementations.
    fn extract_imports(
        module_name: &str,
        modules: &HashMap<String, MModule<WB>>,
        wit: &MITInterfaces<'_>,
        start_index: usize,
    ) -> MResult<HashMap<usize, WITFunction<WB>>> {
        wit.imports()
            .filter(|import|
                // filter out imports that have implementations
                matches!(wit.adapter_types_by_core_type(import.function_type), Some(_)))
            .enumerate()
            .map(|(idx, import)| match modules.get(import.namespace) {
                Some(module) => {
                    use wasmer_it::ast::Type;
                    let (arguments, output_types) =
                        match wit.type_by_idx_r(import.function_type - 2)? {
                            Type::Function {
                                arguments,
                                output_types,
                            } => (arguments.clone(), output_types.clone()),
                            ty => {
                                return Err(MError::IncorrectWIT(format!(
                                    "IT should has Type::Function, but {:?} met",
                                    ty
                                )))
                            }
                        };

                    let func = WITFunction::from_import(
                        module,
                        module_name,
                        import.name,
                        arguments,
                        output_types,
                    )?;

                    Ok((start_index + idx as usize, func))
                }
                None => Err(MError::NoSuchModule(import.namespace.to_string())),
            })
            .collect::<MResult<HashMap<_, _>>>()
    }

    fn extract_memories(
        wasmer_instance: &<WB as WasmBackend>::Instance,
    ) -> Vec<<WB as WasmBackend>::WITMemory> {
        use marine_wasm_backend_traits::Export::Memory;

        let mut memories = wasmer_instance
            .export_iter()
            .filter_map(|(_, export)| match export {
                Memory(memory) => Some(<WB as WasmBackend>::WITMemory::new(memory)),
                _ => None,
            })
            .collect::<Vec<_>>();

        if let Some(Memory(memory)) = wasmer_instance.import_object().get_memory_env()
        //.maybe_with_namespace("env", |env| env.get_export("memory"))
        {
            memories.push(<WB as WasmBackend>::WITMemory::new(memory));
        }

        memories
    }

    fn extract_record_types(wit: &MITInterfaces<'_>) -> MRecordTypes {
        let (record_types_by_id, _) = wit.types().fold(
            (HashMap::new(), 0u64),
            |(mut record_types_by_id, id), ty| {
                match ty {
                    ITAstType::Record(record_type) => {
                        record_types_by_id.insert(id, record_type.clone());
                    }
                    ITAstType::Function { .. } => {}
                };
                (record_types_by_id, id + 1)
            },
        );

        record_types_by_id
    }
}

impl<'v, WB: WasmBackend>
    wasm::structures::Instance<
        ITExport,
        WITFunction<WB>,
        <WB as WasmBackend>::WITMemory,
        <WB as WasmBackend>::WITMemoryView,
    > for ITInstance<WB>
{
    fn export(&self, _export_name: &str) -> Option<&ITExport> {
        // exports aren't used in this version of IT
        None
    }

    fn local_or_import<I: TypedIndex + LocalImportIndex>(
        &self,
        index: I,
    ) -> Option<&WITFunction<WB>> {
        self.funcs.get(&index.index())
    }

    fn memory(&self, index: usize) -> Option<&<WB as WasmBackend>::WITMemory> {
        if index >= self.memories.len() {
            None
        } else {
            Some(&self.memories[index])
        }
    }

    fn memory_view(&self, index: usize) -> Option<<WB as WasmBackend>::WITMemoryView> {
        if index >= self.memories.len() {
            return None;
        }

        let memory = &self.memories[index];
        let view: <WB as WasmBackend>::WITMemoryView = memory.view();
        Some(view)
    }

    fn wit_record_by_id(&self, index: u64) -> Option<&Rc<IRecordType>> {
        self.record_types_by_id.get(&index)
    }
}
