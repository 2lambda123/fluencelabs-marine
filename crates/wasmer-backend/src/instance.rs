use anyhow::anyhow;
use wasmer::{AsStoreMut, Extern};
use crate::{
    func_sig_to_function_type, function_type_to_func_sig, wasmer_ty_to_generic_ty, WasmerBackend,
    WasmerContextMut, WasmerFunction, WasmerMemory, WasmerStore,
};

use marine_wasm_backend_traits::*;

pub struct WasmerInstance {
    pub(crate) inner: wasmer::Instance,
}

impl Instance<WasmerBackend> for WasmerInstance {
    fn export_iter<'a>(
        &'a self,
        mut store: WasmerContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<WasmerBackend>)> + 'a> {
        let iter =
            self.inner
                .exports
                .iter()
                .map(move |(name, export): (&String, &wasmer::Extern)| {
                    let export = match export {
                        wasmer::Extern::Function(function) => Export::Function(
                            WasmerFunction::from_func(&mut store, function.clone()),
                        ),
                        wasmer::Extern::Memory(memory) => Export::Memory(memory.clone().into()),
                        _ => Export::Other,
                    };

                    (name.as_str(), export)
                });
        Box::new(iter)
    }

    fn get_nth_memory(
        &self,
        _store: &mut impl AsContextMut<WasmerBackend>,
        memory_index: u32,
    ) -> Option<WasmerMemory> {
        self.inner
            .exports
            .iter()
            .filter_map(|(name, export)| match export {
                Extern::Memory(memory) => Some(memory),
                _ => None,
            }) // todo is there a way to make it better?
            .nth(memory_index as usize)
            .map(|memory| WasmerMemory {
                inner: memory.clone(),
            })
    }

    fn get_memory(
        &self,
        _store: &mut impl AsContextMut<WasmerBackend>,
        name: &str,
    ) -> ResolveResult<WasmerMemory> {
        self.inner
            .exports
            .get_memory(name)
            .map_err(|e| ResolveError::Other(anyhow!(e))) // todo make detailed
            .map(|memory| WasmerMemory {
                inner: memory.clone(),
            })
    }

    fn get_function(
        &self,
        store: &mut impl AsContextMut<WasmerBackend>,
        name: &str,
    ) -> ResolveResult<<WasmerBackend as WasmBackend>::Function> {
        let owner_memory = self
            .inner
            .exports
            .iter()
            .filter_map(|(name, export)| match export {
                Extern::Memory(memory) => Some(memory),
                _ => None,
            }) // todo is there a way to make it better?
            .nth(0)
            .map(Clone::clone); // todo cache memories and export in the instance

        self.inner
            .exports
            .get_function(name)
            .map_err(|e| ResolveError::Other(anyhow!("wasmer cannot find function {}", e))) // todo make detailed
            .map(|func| {
                let ty = func.ty(&store.as_context());

                WasmerFunction {
                    sig: function_type_to_func_sig(&ty),
                    inner: func.clone(),
                    owner_memory,
                }
            })
    }
}