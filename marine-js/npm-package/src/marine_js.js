/*
 * Copyright 2022 Fluence Labs Limited
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

// This is patched generated by wasm-pack file

import { create_wasi, generate_wasi_imports, bind_to_instance } from './snippets/marine-js-backend-8985bcc66aeb2a35/js/wasi_bindings.js';

export async function init(module) {
    let wasm;

    const heap = new Array(128).fill(undefined);

    heap.push(undefined, null, true, false);

    let heap_next = heap.length;

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

        heap[idx] = obj;
        return idx;
    }

    function getObject(idx) { return heap[idx]; }

    function dropObject(idx) {
        if (idx < 132) return;
        heap[idx] = heap_next;
        heap_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropObject(idx);
        return ret;
    }

    const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachedFloat64Memory0 = null;

    function getFloat64Memory0() {
        if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
            cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
        }
        return cachedFloat64Memory0;
    }

    let cachedUint8Memory0 = new Uint8Array();

    function getUint8Memory0() {
        if (cachedUint8Memory0.byteLength === 0) {
            cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    let WASM_VECTOR_LEN = 0;

    const cachedTextEncoder = new TextEncoder('utf-8');

    const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
        ? function (arg, view) {
            return cachedTextEncoder.encodeInto(arg, view);
        }
        : function (arg, view) {
            const buf = cachedTextEncoder.encode(arg);
            view.set(buf);
            return {
                read: arg.length,
                written: buf.length
            };
        });

    function passStringToWasm0(arg, malloc, realloc) {

        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length);
            getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len);

        const mem = getUint8Memory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3);
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachedInt32Memory0 = new Int32Array();


    function makeMutClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1, dtor };
        const real = (...args) => {
            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return f(a, state.b, ...args);
            } finally {
                if (--state.cnt === 0) {
                    wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

                } else {
                    state.a = a;
                }
            }
        };
        real.original = state;

        return real;
    }

    function logError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            let error = (function () {
                try {
                    return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
                } catch(_) {
                    return "<failed to stringify thrown value>";
                }
            }());
            console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
            throw e;
        }
    }

    let stack_pointer = 128;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }

    function __wbg_adapter_36(arg0, arg1, arg2) {
        try {
            const ret = wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__heabadbed476d1dfa(arg0, arg1, addBorrowedObject(arg2));
            return takeObject(ret);
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }

    function getInt32Memory0() {
        if (cachedInt32Memory0.byteLength === 0) {
            cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachedInt32Memory0;
    }

    function handleError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    function _assertNum(n) {
        if (typeof(n) !== 'number') throw new Error('expected a number argument');
    }

    function _assertBoolean(n) {
        if (typeof(n) !== 'boolean') {
            throw new Error('expected a boolean argument');
        }
    }


    function passArray8ToWasm0(arg, malloc) {
        const ptr = malloc(arg.length * 1);
        getUint8Memory0().set(arg, ptr / 1);
        WASM_VECTOR_LEN = arg.length;
        return ptr;
    }

    function passArrayJsValueToWasm0(array, malloc) {
        const ptr = malloc(array.length * 4) >>> 0;
        const mem = getUint32Memory0();
        for (let i = 0; i < array.length; i++) {
            mem[ptr / 4 + i] = addHeapObject(array[i]);
        }
        WASM_VECTOR_LEN = array.length;
        return ptr;
    }
    let cachedUint32Memory0 = null;

    function getUint32Memory0() {
        if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
            cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
        }
        return cachedUint32Memory0;
    }

    /**
     * Registers a module inside web-runtime.
     *
     * # Arguments
     *
     * * `name` - name of module to register
     * * `wit_section_bytes` - bytes of "interface-types" custom section from wasm file
     * * `instance` - `WebAssembly::Instance` made from target wasm file
     *
     * # Return value
     *
     * JSON object with field "error". If error is empty, module is registered.
     * otherwise, it contains error message.
     * @param {string} name
     * @param {Uint8Array} wit_section_bytes
     * @param {any} wasm_instance
     * @returns {string}
     */
    function register_module(config, log_fn) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.register_module(retptr, addHeapObject(config), addHeapObject(log_fn));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }

    /**
     *  Calls a function from a module.
     *
     * # Arguments
     *
     * * module_name - name of registered module
     * * function_name - name of the function to call
     * * args - JSON array of function arguments
     *
     * # Return value
     *
     * JSON object with fields "error" and "result". If "error" is empty string,
     * "result" contains a function return value. Otherwise, "error" contains error message.
     * @param {string} module_name
     * @param {string} function_name
     * @param {string} args
     * @returns {string}
     */
    function call_module(module_name, function_name, args) {
        let deferred5_0;
        let deferred5_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(module_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(function_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(args, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len2 = WASM_VECTOR_LEN;
            wasm.call_module(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            var ptr4 = r0;
            var len4 = r1;
            if (r3) {
                ptr4 = 0; len4 = 0;
                throw takeObject(r2);
            }
            deferred5_0 = ptr4;
            deferred5_1 = len4;
            return getStringFromWasm0(ptr4, len4);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred5_0, deferred5_1);
        }
    }

    function getArrayU8FromWasm0(ptr, len) {
        return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
    }

    function __wbg_get_imports() {
        const imports = {};
        imports.wbg = {};

        imports.wbg.__wbg_newwithargs_3be36855b48d969f = function(arg0, arg1, arg2, arg3) {
            const ret = new Function(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_bind_7d5ce7224bedd5b8 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).bind(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };

        imports.wbg.__wbg_iterator_7c7e58f62eb84700 = function() {
            const ret = Symbol.iterator;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_get_f53c921291c381bd = function() {
            return handleError(function(arg0, arg1) {
                const ret = Reflect.get(getObject(arg0), getObject(arg1));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbindgen_is_function = function(arg0) {
            const ret = typeof (getObject(arg0)) === "function";
            return ret;
        };

        imports.wbg.__wbg_call_557a2f2deacc4912 = function() {
            return handleError(function(arg0, arg1) {
                const ret = getObject(arg0).call(getObject(arg1));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbindgen_is_object = function(arg0) {
            const val = getObject(arg0);
            const ret = typeof val === "object" && val !== null;
            return ret;
        };

        imports.wbg.__wbg_next_f4bc0e96ea67da68 = function(arg0) {
            const ret = getObject(arg0).next;
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_memory = function() {
            const ret = wasm.memory;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_buffer_55ba7a6b1b92e2ac = function(arg0) {
            const ret = getObject(arg0).buffer;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_new_09938a7d020f049b = function(arg0) {
            const ret = new Uint8Array(getObject(arg0));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_set_3698e3ca519b3c3c = function(arg0, arg1, arg2) {
            getObject(arg0).set(getObject(arg1), arg2 >>> 0);
        };

        imports.wbg.__wbg_length_0aab7ffd65ad19ed = function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        };

        imports.wbg.__wbg_newwithbyteoffsetandlength_88d1d8be5df94b9b = function(arg0, arg1, arg2) {
            const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
            const ret = new Error(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_setindex_f47cfb913f6e49c8 = function(arg0, arg1, arg2) {
            getObject(arg0)[arg1 >>> 0] = arg2;
        };

        imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_getwithrefkey_5e6d9547403deab8 = function(arg0, arg1) {
            const ret = getObject(arg0)[getObject(arg1)];
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_is_undefined = function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        };

        imports.wbg.__wbindgen_in = function(arg0, arg1) {
            const ret = getObject(arg0) in getObject(arg1);
            return ret;
        };

        imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
            const ret = getObject(arg0) == getObject(arg1);
            return ret;
        };

        imports.wbg.__wbg_next_ec061e48a0e72a96 = function() {
            return handleError(function(arg0) {
                const ret = getObject(arg0).next();
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_done_b6abb27d42b63867 = function(arg0) {
            const ret = getObject(arg0).done;
            return ret;
        };

        imports.wbg.__wbg_value_2f4ef2036bfad28e = function(arg0) {
            const ret = getObject(arg0).value;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_get_7303ed2ef026b2f5 = function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_entries_13e011453776468f = function(arg0) {
            const ret = Object.entries(getObject(arg0));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_length_820c786973abdd8a = function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        };

        imports.wbg.__wbg_isArray_04e59fb73f78ab5b = function(arg0) {
            const ret = Array.isArray(getObject(arg0));
            return ret;
        };

        imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof obj === "number" ? obj : undefined;
            getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        };

        imports.wbg.__wbg_isSafeInteger_2088b01008075470 = function(arg0) {
            const ret = Number.isSafeInteger(getObject(arg0));
            return ret;
        };

        imports.wbg.__wbg_newwithlength_89eeca401d8918c2 = function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_new_5e81aea3ac1b5637 = function() {
            return handleError(function(arg0) {
                const ret = new WebAssembly.Module(getObject(arg0));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_new_0f2b71ca2f2a6029 = function() {
            const ret = new Map();
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_set_da7be7bf0e037b14 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_createwasi_9079145d98d65af4 = function(arg0) {
            const ret = create_wasi(takeObject(arg0));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_byteLength_1a59a59856fc656a = function(arg0) {
            const ret = getObject(arg0).byteLength;
            return ret;
        };

        imports.wbg.__wbg_getindex_ed5eaef94b3df248 = function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return ret;
        };

        imports.wbg.__wbg_new_0394642eae39db16 = function() {
            const ret = new Array();
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_apply_46ea2bb0ad750196 = function() {
            return handleError(function(arg0, arg1, arg2) {
                const ret = Reflect.apply(getObject(arg0), getObject(arg1), getObject(arg2));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_log_c9bb086ced3cfca3 = function(arg0, arg1) {
            console.log(getObject(arg0), getObject(arg1));
        };

        imports.wbg.__wbg_new_2b6fea4ea03b1b95 = function() {
            const ret = new Object();
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_set_841ac57cff3d672b = function(arg0, arg1, arg2) {
            getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
        };

        imports.wbg.__wbg_push_109cfc26d02582dd = function(arg0, arg1) {
            const ret = getObject(arg0).push(getObject(arg1));
            return ret;
        };

        imports.wbg.__wbg_error_75cacb398853d810 = function(arg0, arg1) {
            console.error(getObject(arg0), getObject(arg1));
        };

        imports.wbg.__wbg_log_53ed96ea72ace5e9 = function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        };

        imports.wbg.__wbg_error_93b671ae91baaee7 = function(arg0, arg1) {
            console.error(getStringFromWasm0(arg0, arg1));
        };

        imports.wbg.__wbg_warn_52c5b3e773c3a056 = function(arg0, arg1) {
            console.warn(getStringFromWasm0(arg0, arg1));
        };

        imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof obj === "string" ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };

        imports.wbg.__wbg_generatewasiimports_6af0910cd0fc37fa = function(arg0, arg1) {
            const ret = generate_wasi_imports(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_set_07da13cc24b69217 = function() {
            return handleError(function(arg0, arg1, arg2) {
                const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
                return ret;
            }, arguments);
        };

        imports.wbg.__wbg_log_dc06ec929fc95a20 = function(arg0) {
            console.log(getObject(arg0));
        };

        imports.wbg.__wbg_new_d5513c182d79afcb = function() {
            return handleError(function(arg0, arg1) {
                const ret = new WebAssembly.Instance(getObject(arg0), getObject(arg1));
                return addHeapObject(ret);
            }, arguments);
        };

        imports.wbg.__wbg_bindtoinstance_84132e959c03c76d = function(arg0, arg1) {
            bind_to_instance(getObject(arg0), getObject(arg1));
        };

        imports.wbg.__wbg_exports_311291a1333429a3 = function(arg0) {
            const ret = getObject(arg0).exports;
            return addHeapObject(ret);
        };

        imports.wbg.__wbg_instanceof_Memory_331618ccd3fa615d = function(arg0) {
            let result;

            try {
                result = getObject(arg0) instanceof WebAssembly.Memory;
            } catch {
                result = false;
            }

            const ret = result;
            return ret;
        };

        imports.wbg.__wbg_subarray_d82be056deb4ad27 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_number_new = function(arg0) {
            const ret = arg0;
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
            const ret = arg0;
            return addHeapObject(ret);
        };

        imports.wbg.__wbindgen_boolean_get = function(arg0) {
            const v = getObject(arg0);
            const ret = typeof v === "boolean" ? (v ? 1 : 0) : 2;
            return ret;
        };

        imports.wbg.__wbg_instanceof_Uint8Array_1349640af2da2e88 = function(arg0) {
            let result;

            try {
                result = getObject(arg0) instanceof Uint8Array;
            } catch {
                result = false;
            }

            const ret = result;
            return ret;
        };

        imports.wbg.__wbg_instanceof_ArrayBuffer_ef2632aa0d4bfff8 = function(arg0) {
            let result;

            try {
                result = getObject(arg0) instanceof ArrayBuffer;
            } catch {
                result = false;
            }

            const ret = result;
            return ret;
        };

        imports.wbg.__wbg_String_88810dfeb4021902 = function(arg0, arg1) {
            const ret = String(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };

        imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };

        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };

        imports.wbg.__wbindgen_closure_wrapper104 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 109, __wbg_adapter_36);
            return addHeapObject(ret);
        };

        return imports;
    }

    function __wbg_init_memory(imports, maybe_memory) {

    }

    function __wbg_finalize_init(instance, module) {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        cachedInt32Memory0 = new Int32Array();
        cachedUint8Memory0 = new Uint8Array();

        // calls main() function. Used to set up
        wasm.__wbindgen_start();
        return wasm;
    }

    async function __wbg_init(wasmModule) {
        const imports = __wbg_get_imports();
        __wbg_init_memory(imports);
        const instance = await WebAssembly.instantiate(wasmModule, imports);

        return __wbg_finalize_init(instance, module);
    }

    await __wbg_init(module);

    return {
        wasm: wasm,
        register_module,
        call_module,
    };
}
