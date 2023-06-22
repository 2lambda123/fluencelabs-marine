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
use crate::JsContext;
use crate::JsContextMut;
use crate::JsInstance;
use crate::JsWasmBackend;

use marine_wasm_backend_traits::impl_for_each_function_signature;
use marine_wasm_backend_traits::replace_with;
use marine_wasm_backend_traits::prelude::*;

use anyhow::anyhow;

pub struct JsCaller {
    pub(crate) store_inner: *mut crate::store::JsStoreInner,
    pub(crate) caller_instance: Option<JsInstance>,
}

impl Caller<JsWasmBackend> for JsCaller {
    fn memory(&mut self, memory_index: u32) -> Option<<JsWasmBackend as WasmBackend>::Memory> {
        self.caller_instance
            .clone()
            .and_then(|instance| instance.get_nth_memory(&mut self.as_context_mut(), memory_index))
    }
}

impl AsContext<JsWasmBackend> for JsCaller {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        JsContext::from_raw_ptr(self.store_inner)
    }
}

impl AsContextMut<JsWasmBackend> for JsCaller {
    fn as_context_mut(&mut self) -> <JsWasmBackend as WasmBackend>::ContextMut<'_> {
        JsContextMut::from_raw_ptr(self.store_inner)
    }
}

/// Generates a function that accepts an Fn with $num template parameters and turns it into JsFunction.
/// Needed to allow users to pass almost any function to `Function::new_typed` without worrying about signature.
macro_rules! impl_func_getter {
    ($num:tt $($args:ident)*) => (paste::paste!{
        #[allow(unused_parens)]
        impl FuncGetter<JsWasmBackend, ($(replace_with!($args -> i32)),*), ()> for JsCaller {
            fn get_func(
                &mut self,
                name: &str,
            ) -> Result<
                Box<
                    dyn FnMut(&mut JsContextMut<'_>, ($(replace_with!($args -> i32)),*)) -> Result<(), RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                let mut store = JsContextMut::from_raw_ptr(self.store_inner);
                let func = self
                    .caller_instance
                    .as_ref()
                    .ok_or_else(|| ResolveError::Other(anyhow!("Cannot call a function not bound to an instance")))?
                    .get_function(&mut store, name)?;

                let func = move |store: &mut JsContextMut<'_>, ($($args),*)| -> Result<(), RuntimeError> {
                    let args: [WValue; $num] = [$(Into::<WValue>::into($args)),*];
                    let res = func.call(store, &args)?;
                    match res.len() {
                        0 =>  Ok(()),
                        x => Err(RuntimeError::IncorrectResultsNumber{
                            expected: 0,
                            actual: x,
                        })
                    }
                };

                Ok(Box::new(func))
            }
        }

        #[allow(unused_parens)]
        impl FuncGetter<JsWasmBackend, ($(replace_with!($args -> i32)),*), i32> for JsCaller {
            fn get_func(
                &mut self,
                name: &str,
            ) -> Result<
                Box<
                    dyn FnMut(&mut JsContextMut<'_>, ($(replace_with!($args -> i32)),*)) -> Result<i32, RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                let mut store = JsContextMut::from_raw_ptr(self.store_inner);
                let func = self
                    .caller_instance
                    .as_ref()
                    .ok_or_else(|| ResolveError::ExportNotFound(name.to_string()))?
                    .get_function(&mut store, name)?;

                let func = move |store: &mut JsContextMut<'_>, ($($args),*)| -> Result<i32, RuntimeError> {
                    let args: [WValue; $num] = [$(Into::<WValue>::into($args)),*];
                    let res = func.call(store, &args)?;
                    match res.len() {
                        1 =>  Ok(res[0].to_i32()),
                        x => Err(RuntimeError::IncorrectResultsNumber{
                            expected: 1,
                            actual: x,
                        })
                    }
                };

                Ok(Box::new(func))
            }
        }
    });
}

impl_for_each_function_signature!(impl_func_getter);
