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

import {
    call_export,
    read_byte,
    write_byte,
    get_memory_size,
    read_byte_range,
    write_byte_range
} from './snippets/marine-web-runtime-6faa67b8af9cc173/marine-js.js';

export async function init(module) {
    let wasm;

    const heap = new Array(32).fill(undefined);

    heap.push(undefined, null, true, false);

    function getObject(idx) {
        return heap[idx];
    }

    let heap_next = heap.length;

    function dropObject(idx) {
        if (idx < 36) return;
        heap[idx] = heap_next;
        heap_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropObject(idx);
        return ret;
    }

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    let WASM_VECTOR_LEN = 0;

    let cachedTextEncoder = new TextEncoder('utf-8');

    const encodeString =
        typeof cachedTextEncoder.encodeInto === 'function'
            ? function (arg, view) {
                  return cachedTextEncoder.encodeInto(arg, view);
              }
            : function (arg, view) {
                  const buf = cachedTextEncoder.encode(arg);
                  view.set(buf);
                  return {
                      read: arg.length,
                      written: buf.length,
                  };
              };

    function passStringToWasm0(arg, malloc, realloc) {
        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length);
            getUint8Memory0()
                .subarray(ptr, ptr + buf.length)
                .set(buf);
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

    let cachegetInt32Memory0 = null;
    function getInt32Memory0() {
        if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
            cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachegetInt32Memory0;
    }

    function getArrayU8FromWasm0(ptr, len) {
        return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
    }

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }
    /**
     */
    function main() {
        wasm.main();
    }

    function passArray8ToWasm0(arg, malloc) {
        const ptr = malloc(arg.length * 1);
        getUint8Memory0().set(arg, ptr / 1);
        WASM_VECTOR_LEN = arg.length;
        return ptr;
    }
    /**
     * Registers a module insite web-runtime.
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
     * otherwise, it contaits error message.
     * @param {string} name
     * @param {Uint8Array} wit_section_bytes
     * @param {any} wasm_instance
     * @returns {string}
     */
    function register_module(name, wit_section_bytes, wasm_instance) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passArray8ToWasm0(wit_section_bytes, wasm.__wbindgen_malloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.register_module(retptr, ptr0, len0, ptr1, len1, addHeapObject(wasm_instance));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
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
     * "result" contains a function return value. Othervise, "error" contains error message.
     * @param {string} module_name
     * @param {string} function_name
     * @param {string} args
     * @returns {string}
     */
    function call_module(module_name, function_name, args) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = passStringToWasm0(module_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passStringToWasm0(function_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            var ptr2 = passStringToWasm0(args, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len2 = WASM_VECTOR_LEN;
            wasm.call_module(retptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }

    async function init(wasmModule) {
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbg_new_693216e109162396 = function() {
            var ret = new Error();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
            var ret = getObject(arg1).stack;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
            try {
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(arg0, arg1);
            }
        };
        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbg_writebyte_5cf11e3bc7462ec2 = function(arg0, arg1, arg2) {
            write_byte(getObject(arg0), arg1 >>> 0, arg2);
        };
        imports.wbg.__wbg_writebyterange_bca7718185fe74fe = function(arg0, arg1, arg2, arg3) {
            write_byte_range(getObject(arg0), arg1 >>> 0, getArrayU8FromWasm0(arg2, arg3));
        };
        imports.wbg.__wbg_readbyte_4e42fb4a6e94d4cc = function(arg0, arg1) {
            var ret = read_byte(getObject(arg0), arg1 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_readbyterange_a6e4127576d4a165 = function(arg0, arg1, arg2, arg3) {
            read_byte_range(getObject(arg0), arg1 >>> 0, getArrayU8FromWasm0(arg2, arg3));
        };
        imports.wbg.__wbg_getmemorysize_44ed7b542fa6e518 = function(arg0) {
            var ret = get_memory_size(getObject(arg0));
            return ret;
        };
        imports.wbg.__wbg_callexport_a4e71f5003bf3d97 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
            var ret = call_export(getObject(arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5));
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };

        const instance = await WebAssembly.instantiate(wasmModule, imports);
        wasm = instance.exports;

        // strange line from autogenerated code. No idea why it's needed
        init.__wbindgen_wasm_module = module;
        // calls main() function. Used to set up
        wasm.__wbindgen_start();
        return wasm;
    }

    await init(module);

    return {
        wasm: wasm,
        register_module,
        call_module,
    };
}
