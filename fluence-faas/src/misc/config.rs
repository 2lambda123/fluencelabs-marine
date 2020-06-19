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

use crate::FaaSError;

use serde_derive::Deserialize;
use toml::from_slice;

use std::collections::HashMap;

/*
An example of the config:

core_modules_dir = "wasm/artifacts/wasm_modules"

[[core_module]]
    name = "ipfs_node.wasm"
    mem_pages_count = 100
    logger_enabled = true

    [core_module.imports]
    mysql = "/usr/bin/mysql"
    ipfs = "/usr/local/bin/ipfs"

    [core_module.wasi]
    envs = []
    preopened_files = ["/Users/user/tmp/"]
    mapped_dirs = { "tmp" = "/Users/user/tmp" }

[rpc_module]
    mem_pages_count = 100
    logger_enabled = true

    [rpc_module.wasi]
    envs = []
    preopened_files = ["/Users/user/tmp"]
    mapped_dirs = { "tmp" = "/Users/user/tmp" }
 */

#[derive(Deserialize, Debug)]
pub(crate) struct RawCoreModulesConfig {
    pub core_module: Vec<RawModuleConfig>,
    pub rpc_module: Option<RawRPCModuleConfig>,
    pub core_modules_dir: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RawModuleConfig {
    pub name: String,
    pub mem_pages_count: Option<u32>,
    pub logger_enabled: Option<bool>,
    pub imports: Option<toml::value::Table>,
    pub wasi: Option<RawWASIConfig>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RawWASIConfig {
    pub envs: Option<Vec<String>>,
    pub preopened_files: Option<Vec<String>>,
    pub mapped_dirs: Option<toml::value::Table>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RawRPCModuleConfig {
    pub mem_pages_count: Option<u32>,
    pub logger_enabled: Option<bool>,
    pub wasi: Option<RawWASIConfig>,
}

#[derive(Debug)]
pub(crate) struct NodeConfig {
    pub core_modules_dir: String,
    pub modules_config: HashMap<String, ModuleConfig>,
    pub rpc_module_config: Option<ModuleConfig>,
}

#[derive(Debug)]
pub(crate) struct ModuleConfig {
    pub mem_pages_count: Option<u32>,
    pub logger_enabled: Option<bool>,
    pub imports: Option<Vec<(String, String)>>,
    pub wasi: Option<WASIConfig>,
}

#[derive(Debug)]
pub(crate) struct WASIConfig {
    pub envs: Option<Vec<Vec<u8>>>,
    pub preopened_files: Option<Vec<String>>,
    pub mapped_dirs: Option<Vec<(String, String)>>,
}

pub(crate) fn parse_config_from_file(
    config_file_path: std::path::PathBuf,
) -> Result<NodeConfig, FaaSError> {
    let file_content = std::fs::read(config_file_path)?;
    let config: RawCoreModulesConfig =
        from_slice(&file_content).map_err(|err| FaaSError::ConfigParseError(format!("{}", err)))?;

    let modules_config = config
        .core_module
        .into_iter()
        .map(|module| {
            let imports: Option<Vec<(String, String)>> = module.imports.map(|import| {
                import
                    .into_iter()
                    .map(|(import_func_name, host_cmd)| {
                        (import_func_name, host_cmd.try_into::<String>().unwrap())
                    })
                    .collect::<Vec<_>>()
            });

            let wasi = module.wasi.map(parse_raw_wasi);
            (
                module.name,
                ModuleConfig {
                    mem_pages_count: module.mem_pages_count,
                    logger_enabled: module.logger_enabled,
                    imports,
                    wasi,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let rpc_module_config = config.rpc_module.map(|rpc_module| {
        let wasi = rpc_module.wasi.map(parse_raw_wasi);

        ModuleConfig {
            mem_pages_count: rpc_module.mem_pages_count,
            logger_enabled: rpc_module.logger_enabled,
            imports: None,
            wasi,
        }
    });

    Ok(NodeConfig {
        core_modules_dir: config.core_modules_dir,
        modules_config,
        rpc_module_config,
    })
}

fn parse_raw_wasi(wasi: RawWASIConfig) -> WASIConfig {
    let envs = wasi
        .envs
        .map(|env| env.into_iter().map(|e| e.into_bytes()).collect::<Vec<_>>());

    let mapped_dirs = wasi.mapped_dirs.map(|mapped_dir| {
        mapped_dir
            .into_iter()
            .map(|(from, to)| (from, to.try_into::<String>().unwrap()))
            .collect::<Vec<_>>()
    });

    WASIConfig {
        envs,
        preopened_files: wasi.preopened_files,
        mapped_dirs,
    }
}
