use marine_wasm_backend_traits::*;

use crate::{sig_to_fn_ty, StoreState, val_to_wvalue, WasmtimeCaller, WasmtimeContextMut, WasmtimeWasmBackend, wvalue_to_val};
use crate::utils::fn_ty_to_sig;

pub struct WasmtimeFunction {
    pub(crate) inner: wasmtime::Func,
    pub(crate) signature: FuncSig,
}

impl Function<WasmtimeWasmBackend> for WasmtimeFunction {
    fn new<F>(store: &mut impl AsContextMut<WasmtimeWasmBackend>, sig: FuncSig, func: F) -> Self where F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static {
        let ty = sig_to_fn_ty(&sig);
        let func = wasmtime::Func::new(store.as_context_mut().inner, ty, func);
        WasmtimeFunction {
            inner: func,
            signature: sig
        }
    }

    fn new_with_ctx<F>(store: &mut impl AsContextMut<WasmtimeWasmBackend>, sig: FuncSig, func: F) -> Self where F: for<'c> Fn(<WasmtimeWasmBackend as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue> + Sync + Send + 'static {
        let ty = sig_to_fn_ty(&sig);
        let func = wasmtime::Func::new(store.as_context_mut().inner, ty, func);
        WasmtimeFunction {
            inner: func,
            signature: sig
        }
    }

    fn new_typed<Params, Results, Env>(store: &mut impl marine_wasm_backend_traits::AsContextMut<WasmtimeWasmBackend>, func: impl IntoFunc<WasmtimeWasmBackend, Params, Results, Env>) -> Self {
        func.into_func(store)
    }

    fn signature<'c>(&self, store: &mut impl AsContextMut<WasmtimeWasmBackend>) -> &FuncSig {
        &self.signature
    }

    fn call<'c>(&self, store: &mut impl AsContextMut<WasmtimeWasmBackend>, args: &[WValue]) -> CallResult<Vec<WValue>> {
        let args = args.iter().map(wvalue_to_val).collect::<Vec<_>>();

        let mut rets = Vec::new();
        rets.resize(
            self.signature.returns().collect::<Vec<_>>().len(),
            wasmtime::Val::null(),
        ); // todo make O(1), not O(n)
        self.inner.call(store.as_context_mut().inner, &args, &mut rets).unwrap(); // todo handle error
        let rets = rets
            .iter()
            .map(val_to_wvalue)
            .collect::<Result<Vec<_>, ()>>()
            .unwrap(); // todo handle error
        Ok(rets)
    }
}


macro_rules! impl_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_with_env_ $num >] <F>(mut ctx: WasmtimeContextMut<'_>, func: F) -> WasmtimeFunction
            where F: Fn(WasmtimeCaller<'_>, $(replace_with!($args -> i32),)*) -> () + Send + Sync + 'static {

            let func = move |caller: wasmtime::Caller<'_, StoreState>, $($args,)*| {
                let caller = WasmtimeCaller {inner: caller};
                func(caller, $($args,)*)
            };

            let func = wasmtime::Func::wrap(&mut ctx.inner, func);
            use WType::I32;
            let params = vec![$(replace_with!($args -> I32),)*];
            let rets = vec![I32,];
            let sig = FuncSig::new(params, rets);

            WasmtimeFunction {
                signature: sig,
                inner: func
            }
        }

        fn [< new_typed_with_env_ $num _r>] <F>(mut ctx: WasmtimeContextMut<'_>, func: F) -> WasmtimeFunction
            where F: Fn(WasmtimeCaller<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static {

            let func = move |caller: wasmtime::Caller<'_, StoreState>, $($args,)*| -> i32{
                let caller = WasmtimeCaller {inner: caller};
                func(caller, $($args,)*)
            };

            let func = wasmtime::Func::wrap(&mut ctx.inner, func);
            use WType::I32;
            let params = vec![$(replace_with!($args -> I32),)*];
            let rets = vec![I32,];
            let sig = FuncSig::new(params, rets);

            WasmtimeFunction {
                signature: sig,
                inner: func
            }
        }
    });
}

impl FuncConstructor<WasmtimeWasmBackend> for WasmtimeFunction {
    fn new_typed_with_env_0_test<F>(mut ctx: WasmtimeContextMut<'_>, func: F) -> WasmtimeFunction
        where F: Fn(WasmtimeCaller<'_>) -> () + Send + Sync + 'static {
        let func = move |caller: wasmtime::Caller<'_, StoreState>| {
            let caller = WasmtimeCaller {inner: caller};
            func(caller)
        };

        let func = wasmtime::Func::wrap(&mut ctx.inner, func);
        use WType::I32;
        let params = vec![];
        let rets = vec![];
        let sig = FuncSig::new(params, rets);

        WasmtimeFunction {
            signature: sig,
            inner: func
        }
    }

    impl_for_each_function_signature!(impl_func_construction);
}