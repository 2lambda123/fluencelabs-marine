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

#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

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

use wasmtime_wasi::WasiCtx;

const MB: usize = 1024 * 1024;

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
    type ImportCallContext<'c> = WasmtimeImportCallContext<'c>;
    type HostFunction = WasmtimeFunction;
    type ExportFunction = WasmtimeFunction;
    type Memory = WasmtimeMemory;
    type MemoryView = WasmtimeMemory;
    type Wasi = WasmtimeWasi;

    fn new() -> WasmBackendResult<Self> {
        let mut config = wasmtime::Config::new();
        config
            .debug_info(false)
            .wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable)
            .async_support(true)
            .max_wasm_stack(2 * MB);
        let engine =
            wasmtime::Engine::new(&config).map_err(WasmBackendError::InitializationError)?;

        Ok(Self { engine })
    }

    fn new_async() -> WasmBackendResult<Self> {
        let mut config = wasmtime::Config::new();
        config
            .debug_info(false)
            .wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable)
            .async_support(true)
            .epoch_interruption(true)
            .max_wasm_stack(2 * MB);

        let engine =
            wasmtime::Engine::new(&config).map_err(WasmBackendError::InitializationError)?;

        Ok(Self { engine })
    }
}

impl WasmtimeWasmBackend {
    pub fn increment_epoch(&self) {
        self.engine.increment_epoch()
    }

    pub fn new_async_epoch_based() -> WasmBackendResult<Self> {
        let mut config = wasmtime::Config::new();
        config
            .debug_info(false)
            .wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable)
            .async_support(true)
            .epoch_interruption(true)
            .max_wasm_stack(2 * MB);

        let engine =
            wasmtime::Engine::new(&config).map_err(WasmBackendError::InitializationError)?;

        Ok(Self { engine })
    }
}

#[derive(Default)]
pub struct StoreState {
    wasi: Vec<WasiCtx>, // wasmtime store does not release memory until drop, so do we
}
