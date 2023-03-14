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

use crate::AsContextMut;
use crate::FuncSig;
use crate::impl_for_each_function_signature;
use crate::RuntimeResult;
use crate::WasmBackend;
use crate::WValue;

/// A Wasm function handle, it can be either a function from a host or an export from an `Instance`.
/// As it is only a handle to an object in `Store`, cloning is cheap
pub trait Function<WB: WasmBackend>: Send + Sync {
    /// Creates a new function with dynamic signature.
    /// The signature check is performed at runtime.
    fn new<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static;

    /// Creates a new function with dynamic signature that needs a context.
    fn new_with_caller<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(<WB as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
            + Sync
            + Send
            + 'static;

    /// Creates a new function with static signature.
    /// Requires less runtime checks when called.
    fn new_typed<Params, Results, Env>(
        store: &mut impl AsContextMut<WB>,
        func: impl IntoFunc<WB, Params, Results, Env>,
    ) -> Self;

    /// Returns the signature of the function.
    /// The signature is constructed each time this function is called, so
    /// it is not recommended to use this function extensively.
    fn signature(&self, store: &mut impl AsContextMut<WB>) -> FuncSig;

    /// Calls the wasm function.
    /// # Panics:
    ///     If given a store different from the one that stores the function.
    /// # Errors:
    ///     See `RuntimeError` documentation.
    fn call(
        &self,
        store: &mut impl AsContextMut<WB>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>>;
}

/// A helper trait for creating a function with a static signature.
/// Should not be implemented by users.
/// Implemented for all functions that meet the following criteria:
///     * implement Send + Sync + 'static
///     * take or not take Caller as first parameter
///     * take from 0 to 16 i32 parameters
///     * return () or i32
pub trait IntoFunc<WB: WasmBackend, Params, Results, Env> {
    fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function;
}

/// An indicator of using Caller argument.
pub struct WithEnv {}

/// An indicator of using Caller argument.
pub struct WithoutEnv {}

#[macro_export]
macro_rules! replace_with {
    ($from:ident -> $to:ident) => {
        $to
    };
}

macro_rules! impl_into_func {
    ($num:tt $($args:ident)*) => (paste::paste!{
        #[allow(non_snake_case)]
        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), (), WithoutEnv> for F
        where
            WB: WasmBackend,
            F: Fn($(replace_with!($args -> i32),)*) + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function {
                <WB as WasmBackend>::Function:: [< new_typed_ $num >] (ctx.as_context_mut(), self)
            }
        }

        #[allow(non_snake_case)]
        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), (), WithEnv> for F
        where
            WB: WasmBackend,
            F: Fn(<WB as WasmBackend>::Caller<'_>, $(replace_with!($args -> i32),)*) + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function {
                <WB as WasmBackend>::Function:: [< new_typed_with_env_ $num >] (ctx.as_context_mut(), self)
            }
        }

        #[allow(non_snake_case)]
        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), i32, WithoutEnv> for F
        where
            WB: WasmBackend,
            F: Fn($(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function {
                <WB as WasmBackend>::Function:: [< new_typed_ $num _r >] (ctx.as_context_mut(), self)
            }
        }

        #[allow(non_snake_case)]
        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), i32, WithEnv> for F
        where
            WB: WasmBackend,
            F: Fn(<WB as WasmBackend>::Caller<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function {
                <WB as WasmBackend>::Function:: [< new_typed_with_env_ $num _r >] (ctx.as_context_mut(), self)
            }
        }
    });
}

impl_for_each_function_signature!(impl_into_func);

macro_rules! declare_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        #[allow(non_snake_case)]
        fn [< new_typed_ $num >]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::Function
            where F: Fn($(replace_with!($args -> i32),)*) + Send + Sync + 'static
        {
            let func = move |_: <WB as WasmBackend>::Caller<'_>, $($args,)*| { func($($args,)*)};
            Self:: [< new_typed_with_env_ $num >] (ctx, func)
        }

        #[allow(non_snake_case)]
        fn [< new_typed_with_env_ $num >]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::Function
            where F: Fn(<WB as WasmBackend>::Caller<'_>, $(replace_with!($args -> i32),)*) + Send + Sync + 'static;

        #[allow(non_snake_case)]
        fn [< new_typed_ $num _r>]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::Function
            where F: Fn($(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static
        {
            let func = move |_: <WB as WasmBackend>::Caller<'_>, $($args,)*| -> i32 { func($($args,)*)};
            Self:: [< new_typed_with_env_ $num _r >] (ctx, func)
        }

        #[allow(non_snake_case)]
        fn [< new_typed_with_env_ $num _r>]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::Function
            where F: Fn(<WB as WasmBackend>::Caller<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static;
    });
}

pub trait FuncConstructor<WB: WasmBackend> {
    impl_for_each_function_signature!(declare_func_construction);
}
