use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use it_memory_traits::MemoryAccessError;
use marine_wasm_backend_traits::*;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use multimap::MultiMap;
use wasmtime::AsContextMut;
use crate::utils::{val_to_wvalue, val_type_to_wtype, wvalue_to_val};

mod utils;

#[derive(Clone, Default)]
pub struct WasmtimeWasmBackend {
    engine: wasmtime::Engine,
}

impl WasmtimeWasmBackend {
    fn custom_sections(bytes: &[u8]) -> Result<MultiMap<String, Vec<u8>>, String> {
        use wasmparser::{Parser, Payload, Result};
        Parser::new(0)
            .parse_all(bytes)
            .filter_map(|payload| {
                let payload = match payload {
                    Ok(s) => s,
                    Err(e) => return Some(Err(e.to_string())),
                };
                match payload {
                    Payload::CustomSection(reader) => {
                        let name = reader.name().to_string();
                        let data = reader.data().to_vec();
                        Some(Ok((name, data)))
                    }
                    _ => None,
                }
            })
            .collect()
    }
}

impl WasmBackend for WasmtimeWasmBackend {
    type Module = WasmtimeModule;
    type Instance = WasmtimeInstance;
    type Store = WasmtimeStore;
    type ContextMut<'c> = WasmtimeStoreContextMut<'c>;
    type Caller<'c> = WasmtimeCaller<'c>;
    //type StoreContextMut = WasmtimeStoreContextMut;
    //type StoreContextMut = WasmtimeStoreContextMut;
    type ImportObject = WasmtimeImportObject;
    type DynamicFunc = WasmtimeDynamicFunc;
    type MemoryExport = WasmtimeMemoryExport;
    type FunctionExport = WasmtimeFunctionExport;
    type Namespace = WasmtimeNamespace;
    type ExportedDynFunc = WasmtimeExportedDynFunc;
    type WITMemory = WasmtimeWITMemory;
    type WITMemoryView = WasmtimeWITMemoryView;
    type Wasi = WasmtimeWasi;

    fn compile(store: &mut WasmtimeStore, wasm: &[u8]) -> WasmBackendResult<Self::Module> {
        let module = wasmtime::Module::new(store.store.engine(), wasm).unwrap(); //todo convert error;
        let custom_sections = WasmtimeWasmBackend::custom_sections(wasm).unwrap(); //todo convert error;
        Ok(WasmtimeModule {
            custom_sections,
            module,
        })
    }
}

// general
pub struct WasmtimeStore {
    store: wasmtime::Store<()>,
}

impl Store<WasmtimeWasmBackend> for WasmtimeStore {
    fn new(backend: &WasmtimeWasmBackend) -> Self {
        Self {
            store: wasmtime::Store::new(&backend.engine, ()),
        }
    }
}
/*
impl AsStoreContextMut<WasmtimeWasmBackend> for WasmtimeStore {
    fn store_context_mut<'c, CTX: ExportContext<'c, WasmtimeWasmBackend>>(&mut self) -> ContextMut<'c, WasmtimeWasmBackend, WasmtimeExportContext<'c>> {
        ContextMut::Store(&mut self)
    }
}*/


impl marine_wasm_backend_traits::AsContextMut<WasmtimeWasmBackend> for WasmtimeStore {
    fn as_context_mut(&mut self) -> WasmtimeStoreContextMut<'_> {
        WasmtimeStoreContextMut {
            ctx: self.store.as_context_mut()
        }
    }
}

pub struct WasmtimeStoreContextMut<'c> {
    ctx: wasmtime::StoreContextMut<'c, ()>,
}


impl<'c> ContextMut<WasmtimeWasmBackend> for WasmtimeStoreContextMut<'c> {}
//impl StoreContextMut<WasmtimeWasmBackend> for WasmtimeStoreContextMut {}

impl<'c> marine_wasm_backend_traits::AsContextMut<WasmtimeWasmBackend> for WasmtimeStoreContextMut<'c> {
    fn as_context_mut(&mut self) -> WasmtimeStoreContextMut<'_> {
        WasmtimeStoreContextMut {
            ctx: self.ctx.as_context_mut()
        }
    }
}
pub struct WasmtimeCaller<'c> {
    caller: wasmtime::Caller<'c, ()>,
}


impl<'c> Caller<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn memory(&mut self, memory_index: u32) -> <WasmtimeWasmBackend as WasmBackend>::WITMemory {
        let memory = self.caller.get_export("memory").unwrap(); // todo: handle error

        WasmtimeWITMemory {
            memory: memory.into_memory().unwrap() // todo: handle error
        }
    }
}

impl<'c> marine_wasm_backend_traits::AsContextMut<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn as_context_mut(&mut self) -> <WasmtimeWasmBackend as WasmBackend>::ContextMut<'_> {
        WasmtimeStoreContextMut {
            ctx: wasmtime::AsContextMut::as_context_mut(&mut self.caller)
        }

    }
}

pub struct WasmtimeModule {
    custom_sections: MultiMap<String, Vec<u8>>,
    module: wasmtime::Module,
}

impl Module<WasmtimeWasmBackend> for WasmtimeModule {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]> {
        self.custom_sections
            .get_vec(key)
            .map(|value| value.as_slice())
    }

    fn instantiate(
        &self,
        store: &mut WasmtimeStore,
        imports: &WasmtimeImportObject,
    ) -> WasmBackendResult<<WasmtimeWasmBackend as WasmBackend>::Instance> {
        let instance = imports
            .linker
            .instantiate(&mut store.store, &self.module)
            .unwrap(); // todo handle error
        Ok(WasmtimeInstance { instance })
    }
}

pub struct WasmtimeInstance {
    instance: wasmtime::Instance,
}

impl Instance<WasmtimeWasmBackend> for WasmtimeInstance {
    fn export_iter<'a>(
        &'a self,
        store: &'a mut WasmtimeStore,
    ) -> Box<dyn Iterator<Item = (String, Export<WasmtimeMemoryExport, WasmtimeFunctionExport>)> + 'a>
    {
        let iter = self.instance.exports(&mut store.store).map(|export| {
            let name = export.name().to_string();
            let export = match export.into_extern() {
                wasmtime::Extern::Memory(memory) => Export::Memory(WasmtimeMemoryExport { memory }),
                wasmtime::Extern::Func(func) => Export::Function(WasmtimeFunctionExport { func }),
                _ => Export::Other,
            };
            (name, export)
        });

        Box::new(iter)
    }

    fn memory(
        &self,
        store: &mut WasmtimeStore,
        memory_index: u32,
    ) -> <WasmtimeWasmBackend as WasmBackend>::WITMemory {
        let memory = self
            .instance
            .exports(&mut store.store)
            .filter_map(wasmtime::Export::into_memory)
            .nth(memory_index as usize)
            .unwrap(); // todo change api to handle error

        WasmtimeWITMemory { memory }
    }

    fn memory_by_name(
        &self,
        store: &mut WasmtimeStore,
        memory_name: &str,
    ) -> Option<<WasmtimeWasmBackend as WasmBackend>::WITMemory> {
        let memory = self.instance.get_memory(&mut store.store, memory_name);

        memory.map(WasmtimeWITMemory::new)
    }

    fn get_func_no_args_no_rets<'a>(
        &'a self,
        store: &mut WasmtimeStore,
        name: &str,
    ) -> ResolveResult<Box<dyn Fn(&mut WasmtimeStore) -> RuntimeResult<()> + 'a>> {
        let func = self.instance.get_func(&mut store.store, name).unwrap(); // todo handle None
        let typed = func.typed::<(), (), _>(&store.store).unwrap(); // todo handle error
        Ok(Box::new(move |store: &mut WasmtimeStore| {
            Ok(typed.call(&mut store.store, ()).unwrap()) //todo handle error
        }))
    }

    fn get_dyn_func<'a>(
        &'a self,
        store: &mut WasmtimeStore,
        name: &str,
    ) -> ResolveResult<<WasmtimeWasmBackend as WasmBackend>::ExportedDynFunc> {
        let func = self.instance.get_func(&mut store.store, name).unwrap(); // todo handle None
        let ty = func.ty(&store.store);
        let params = ty
            .params()
            .map(|ty| {
                val_type_to_wtype(&ty).unwrap() // todo handle error
            })
            .collect::<Vec<_>>();
        let rets = ty
            .results()
            .map(|ty| {
                val_type_to_wtype(&ty).unwrap() // todo handle error
            })
            .collect::<Vec<_>>();

        let sig = FuncSig::new(params, rets);
        Ok(WasmtimeExportedDynFunc {
            func,
            signature: sig,
        })
    }
}

// imports
#[derive(Clone)]
pub struct WasmtimeImportObject {
    linker: wasmtime::Linker<()>,
}

impl ImportObject<WasmtimeWasmBackend> for WasmtimeImportObject {
    fn new(store: &mut WasmtimeStore) -> Self {
        Self {
            linker: wasmtime::Linker::new(store.store.engine()),
        }
    }

    fn register<S>(
        &mut self,
        module: S,
        namespace: WasmtimeNamespace,
    ) -> Option<Box<dyn LikeNamespace<WasmtimeWasmBackend>>>
    where
        S: Into<String>,
    {
        let module: String = module.into();
        for (name, func) in namespace.functions {
            func(self, &module, &name).unwrap(); // todo handle error
        }

        None // todo handle collisions
    }
}

pub struct WasmtimeNamespace {
    functions: Vec<(
        String,
        Box<dyn FnOnce(&mut WasmtimeImportObject, &str, &str) -> Result<(), String> + 'static>,
    )>,
}

macro_rules! impl_insert_fn {
    ($($name:ident: $arg:ty),* => $rets:ty) => {
        impl InsertFn<WasmtimeWasmBackend, ($($arg,)*), $rets> for WasmtimeNamespace {
            fn insert_fn<F>(&mut self, name: impl Into<String>, func: F)
            where F:'static + Fn(&mut WasmtimeCaller, ($($arg,)*)) -> $rets + std::marker::Send + std::marker::Sync {
                let inserter = move |linker: &mut WasmtimeImportObject, module: &str, name: &str| {
                    let func = move |caller: wasmtime::Caller<'_, ()>, $($name: $arg),*| {
                            let mut ctx = WasmtimeCaller {caller};

                            func(&mut ctx, ($($name,)*))
                    };

                    linker.linker.func_wrap(module, name, func).unwrap(); // todo handle error
                    Ok(())
                };

                self.functions.push((name.into(), Box::new(inserter)))
            }
        }
    };
}

impl_insert_fn!(=> ());
impl_insert_fn!(A: i32 => ());
impl_insert_fn!(A: i32, B: i32 => ());
impl_insert_fn!(A: i32, B: i32, C: i32 => ());
impl_insert_fn!(A: i32, B: i32, C: i32, D: i32 => ());

impl LikeNamespace<WasmtimeWasmBackend> for WasmtimeNamespace {}

impl Namespace<WasmtimeWasmBackend> for WasmtimeNamespace {
    fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    fn insert(&mut self, name: impl Into<String>, func: WasmtimeDynamicFunc) {}
}

pub struct WasmtimeDynamicFunc {}

impl<'a> DynamicFunc<'a, WasmtimeWasmBackend> for WasmtimeDynamicFunc {
    fn new<F>(store: &mut <WasmtimeWasmBackend as WasmBackend>::ContextMut<'_>, sig: FuncSig, func: F) -> Self where F: for<'c> Fn(<WasmtimeWasmBackend as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue> + 'static {
        todo!()
    }
}

//exports

pub struct WasmtimeExports {
    exports: Vec<(String, Export<WasmtimeMemoryExport, WasmtimeFunctionExport>)>,
}

pub struct WasmtimeMemoryExport {
    memory: wasmtime::Memory,
}

impl MemoryExport for WasmtimeMemoryExport {}

pub struct WasmtimeFunctionExport {
    func: wasmtime::Func,
}

impl FunctionExport for WasmtimeFunctionExport {}
/*
pub struct WasmtimeExportContext<'a> {
    caller: Caller<'a, ()>,
}
*/
macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl<'c> FuncGetter<$args, $rets> for WasmtimeCaller<'c> {
            unsafe fn get_func<'s>(
                &'s mut self,
                name: &str,
            ) -> Result<Box<dyn FnMut($args) -> Result<$rets, RuntimeError> + 's>, ResolveError>
            {
                todo!()
            }
        }
    };
}

impl_func_getter!((i32, i32), i32);
impl_func_getter!((i32, i32), ());
impl_func_getter!(i32, i32);
impl_func_getter!(i32, ());
impl_func_getter!((), i32);
impl_func_getter!((), ());
/*
impl<'a, 'r> ExportContext<'a, WasmtimeWasmBackend> for WasmtimeExportContext<'r> {
    fn memory(&mut self, memory_index: u32) -> <WasmtimeWasmBackend as WasmBackend>::WITMemory {
        let memory = self.caller.get_export("memory").unwrap();
        WasmtimeWITMemory::new(memory.into_memory().unwrap()) // handle error
    }

    fn store(&mut self) -> &mut <WasmtimeWasmBackend as WasmBackend>::Store {
        todo!()
    }
}*/
/*
impl<'r> AsStoreContextMut<WasmtimeWasmBackend> for WasmtimeExportContext<'r> {
    fn store_context_mut<'c, CTX: ExportContext<'c, WasmtimeWasmBackend>>(&mut self) -> ContextMut<'c, WasmtimeWasmBackend, WasmtimeExportContext<'c>>{
        ContextMut::ExportCtx(&mut self)
    }
}
*/

impl<'c> wasmtime::AsContext for WasmtimeStoreContextMut<'c> {
    type Data = ();

    fn as_context(&self) -> wasmtime::StoreContext<'_, Self::Data> {
        self.ctx.as_context()
    }
}
impl<'c> wasmtime::AsContextMut for WasmtimeStoreContextMut<'c> {
    fn as_context_mut(&mut self) -> wasmtime::StoreContextMut<'_, Self::Data> {
        self.ctx.as_context_mut()
    }
}

pub struct WasmtimeExportedDynFunc {
    func: wasmtime::Func,
    signature: FuncSig,
}


impl ExportedDynFunc<WasmtimeWasmBackend> for WasmtimeExportedDynFunc {
    fn signature<'c>(&self, store: WasmtimeStoreContextMut) -> &FuncSig {
        &self.signature
    }

    fn call<'c>(&self, store: WasmtimeStoreContextMut, args: &[WValue]) -> CallResult<Vec<WValue>> {
        let args = args.iter().map(wvalue_to_val).collect::<Vec<_>>();

        let mut rets = Vec::new();
        rets.resize(
            self.signature.returns().collect::<Vec<_>>().len(),
            wasmtime::Val::null(),
        ); // todo make O(1), not O(n)
        self.func.call(store, &args, &mut rets).unwrap(); // todo handle error
        let rets = rets
            .iter()
            .map(val_to_wvalue)
            .collect::<Result<Vec<_>, ()>>()
            .unwrap(); // todo handle error
        Ok(rets)
    }
}

// Interface types
#[derive(Clone)]
pub struct WasmtimeWITMemory {
    memory: wasmtime::Memory,
}

impl WasmtimeWITMemory {
    fn new(memory: wasmtime::Memory) -> Self {
        Self { memory }
    }
}

impl it_memory_traits::Memory<WasmtimeWITMemoryView> for WasmtimeWITMemory {
    fn view(&self) -> WasmtimeWITMemoryView {
        todo!()
    }
}

impl Memory<WasmtimeWasmBackend> for WasmtimeWITMemory {
    fn new(export: <WasmtimeWasmBackend as WasmBackend>::MemoryExport) -> Self {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }
}

pub struct WasmtimeWITMemoryView {}

impl it_memory_traits::MemoryReadable for WasmtimeWITMemoryView {
    fn read_byte(&self, offset: u32) -> u8 {
        todo!()
    }

    fn read_array<const COUNT: usize>(&self, offset: u32) -> [u8; COUNT] {
        todo!()
    }

    fn read_vec(&self, offset: u32, size: u32) -> Vec<u8> {
        todo!()
    }
}

impl it_memory_traits::MemoryWritable for WasmtimeWITMemoryView {
    fn write_byte(&self, offset: u32, value: u8) {
        todo!()
    }

    fn write_bytes(&self, offset: u32, bytes: &[u8]) {
        todo!()
    }
}

impl it_memory_traits::MemoryView for WasmtimeWITMemoryView {
    fn check_bounds(&self, offset: u32, size: u32) -> Result<(), MemoryAccessError> {
        todo!()
    }
}

// Wasi

pub struct WasmtimeWasi {}

impl WasiImplementation<WasmtimeWasmBackend> for WasmtimeWasi {
    fn generate_import_object_for_version(
        version: WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<Vec<u8>>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<<WasmtimeWasmBackend as WasmBackend>::ImportObject, String> {
        todo!()
    }

    fn get_wasi_state<'s>(
        instance: &'s mut <WasmtimeWasmBackend as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's> {
        todo!()
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
