/*
 * Copyright 2024 Fluence Labs Limited
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

use crate::AppService;
use crate::AppServiceConfig;

use marine_wasm_backend_traits::WasmBackend;
use marine_wasm_backend_traits::WasmBackendResult;
use marine_wasmtime_backend::WasmtimeWasmBackend;

use std::collections::HashMap;

#[derive(Clone)]
pub struct AppServiceFactory<WB: WasmBackend> {
    backend: WB,
}

pub struct EpochTicker(WasmtimeWasmBackend);

impl AppServiceFactory<WasmtimeWasmBackend> {
    pub fn new() -> WasmBackendResult<(Self, EpochTicker)> {
        let backend = WasmtimeWasmBackend::new_async()?;
        Ok(Self::new_with_backend(backend))
    }

    pub fn new_with_backend(backend: WasmtimeWasmBackend) -> (Self, EpochTicker) {
        let ticker = EpochTicker(backend.clone());
        let factory = Self { backend };
        (factory, ticker)
    }

    pub async fn new_app_service<S>(
        &self,
        config: AppServiceConfig,
        service_id: S,
        envs: HashMap<String, String>,
    ) -> crate::Result<AppService>
    where
        S: Into<String>,
    {
        AppService::new_with_backend(self.backend.clone(), config, service_id, envs).await
    }

    #[cfg(feature = "raw-module-api")]
    pub async fn new_app_service_empty_facade<S>(
        &self,
        config: AppServiceConfig,
        service_id: S,
        envs: HashMap<String, String>,
    ) -> crate::Result<AppService>
    where
        S: Into<String>,
    {
        AppService::new_with_empty_facade(self.backend.clone(), config, service_id, envs).await
    }
}

impl EpochTicker {
    pub fn increment_epoch(&self) {
        self.0.increment_epoch()
    }
}