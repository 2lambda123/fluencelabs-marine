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

use crate::ModuleCreationResult;
use crate::InstantiationResult;
use crate::WasmBackend;

use futures::future::BoxFuture;

/// A handle to compiled wasm module.
pub trait Module<WB: WasmBackend>: Sized {
    /// Compiles a wasm bytes into a module and extracts custom sections.
    fn new(store: &mut <WB as WasmBackend>::Store, wasm: &[u8]) -> ModuleCreationResult<Self>;

    /// Returns custom sections corresponding to `name`, empty slice if there is no sections.
    fn custom_sections(&self, name: &str) -> &[Vec<u8>];

    /// Instantiates module by allocating memory, VM state and linking imports with ones from `import` argument.
    /// Does not call `_start` or `_initialize` functions.
    ///
    /// # Panics:
    ///
    ///     If the `Store` given is not the same with `Store` used to create `Imports` and this object.
    fn instantiate<'args>(
        &'args self,
        store: &'args mut <WB as WasmBackend>::Store,
        imports: &'args <WB as WasmBackend>::Imports,
    ) -> BoxFuture<'args, InstantiationResult<<WB as WasmBackend>::Instance>>;
}
