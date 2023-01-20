mod caller;
mod store;
mod utils;
mod module;
mod instance;
mod wasi;
mod function;
mod imports;
mod memory;

use store::*;
use caller::*;
use module::*;
use instance::*;
use wasi::*;
use function::*;
use memory::*;
use imports::*;

use marine_wasm_backend_traits::*;

use multimap::MultiMap;

use crate::utils::{sig_to_fn_ty, val_to_wvalue, wvalue_to_val};

use wasmtime_wasi::WasiCtx;

#[derive(Clone, Default)]
pub struct WasmtimeWasmBackend {
    engine: wasmtime::Engine,
}

impl WasmtimeWasmBackend {
    fn custom_sections(bytes: &[u8]) -> Result<MultiMap<String, Vec<u8>>, String> {
        use wasmparser::{Parser, Payload};
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
    type Context<'c> = WasmtimeContext<'c>;
    type ContextMut<'c> = WasmtimeContextMut<'c>;
    type Caller<'c> = WasmtimeCaller<'c>;
    type Imports = WasmtimeImports;
    type Function = WasmtimeFunction;
    type Memory = WasmtimeMemory;
    type MemoryView = WasmtimeMemory;
    type Wasi = WasmtimeWasi;

    fn compile(store: &mut WasmtimeStore, wasm: &[u8]) -> CompilationResult<Self::Module> {
        let module = wasmtime::Module::new(store.inner.engine(), wasm)
            .map_err(|e| CompilationError::FailedToCompileWasm(e))?; // todo make mode detailed
        let custom_sections = WasmtimeWasmBackend::custom_sections(wasm)
            .map_err(|e| CompilationError::FailedToExtractCustomSections(e))?; // todo make more deatailed

        Ok(WasmtimeModule {
            custom_sections,
            inner: module,
        })
    }
}

#[derive(Default)]
pub struct StoreState {
    wasi: Vec<WasiCtx>, //todo switch to Pool or something
}