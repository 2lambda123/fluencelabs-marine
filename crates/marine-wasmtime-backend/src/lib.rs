/*
 * Copyright 2023 Fluence Labs Limited
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
use utils::*;

use marine_wasm_backend_traits::prelude::*;
use marine_wasm_backend_traits::impl_utils::custom_sections;

use wasmtime_wasi::WasiCtx;

#[derive(Clone, Default)]
pub struct WasmtimeWasmBackend {
    engine: wasmtime::Engine,
}

impl WasmBackend for WasmtimeWasmBackend {
    type Store = WasmtimeStore;
    type Module = WasmtimeModule;
    type Imports = WasmtimeImports;
    type Instance = WasmtimeInstance;
    type Context<'c> = WasmtimeContext<'c>;
    type ContextMut<'c> = WasmtimeContextMut<'c>;
    type Caller<'c> = WasmtimeCaller<'c>;
    type Function = WasmtimeFunction;
    type Memory = WasmtimeMemory;
    type MemoryView = WasmtimeMemory;
    type Wasi = WasmtimeWasi;

    fn compile(store: &mut WasmtimeStore, wasm: &[u8]) -> CompilationResult<Self::Module> {
        let module = wasmtime::Module::new(store.inner.engine(), wasm)
            .map_err(CompilationError::FailedToCompileWasm)?;
        let custom_sections =
            custom_sections(wasm) // TODO: avoid double module parsing
                .map_err(CompilationError::FailedToExtractCustomSections)?;

        Ok(WasmtimeModule {
            custom_sections,
            inner: module,
        })
    }

    fn new() -> Self {
        let mut config = wasmtime::Config::new();
        config
            .debug_info(false)
            .wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        let engine = wasmtime::Engine::new(&config).unwrap();
        Self { engine }
    }
}

#[derive(Default)]
pub struct StoreState {
    wasi: Vec<WasiCtx>, // wasmtime store does not release memory until drop, so do we
}