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

use std::cell::Cell;
use std::ops::Deref;
use wasmer_it::interpreter::wasm;
use wasmer_core::memory::{Memory, MemoryView};
use wasmer_core::vm::LocalMemory;
use crate::module::WasmerSequentialReader;

use crate::module::WasmerSequentialWriter;

use it_traits::{MemoryAccessError, SequentialReader, SequentialWriter};

pub(crate) struct WITMemoryView<'a>(pub(crate) MemoryView<'a, u8>);

#[derive(Clone)]
pub(crate) struct WITMemory(pub(super) Memory);
impl std::ops::Deref for WITMemory {
    type Target = Memory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WITMemoryView<'_> {
    fn check_bounds(
        &self,
        offset: usize,
        size: usize,
        memory_size: usize,
    ) -> Result<(), MemoryAccessError> {
        if offset + size >= memory_size {
            Err(MemoryAccessError::OutOfBounds {
                offset,
                size,
                memory_size,
            })
        } else {
            Ok(())
        }
    }
}

impl wasm::structures::MemoryView for WITMemoryView<'_> {
    fn sequential_writer<'s>(
        &'s self,
        offset: usize,
        size: usize,
    ) -> Result<Box<dyn SequentialWriter + 's>, MemoryAccessError> {
        let view = &self.0;
        let slice = view.deref();

        self.check_bounds(offset, size, slice.len())?;

        let writer = WasmerSequentialWriter {
            offset,
            slice,
            current_offset: Cell::new(offset),
        };

        Ok(Box::new(writer))
    }

    fn sequential_reader<'s>(
        &'s self,
        offset: usize,
        size: usize,
    ) -> Result<Box<dyn SequentialReader + 's>, MemoryAccessError> {
        let view = &self.0;
        let slice: &[Cell<u8>] = view.deref();

        self.check_bounds(offset, size, slice.len())?;

        let reader = WasmerSequentialReader {
            memory: slice,
            offset: Cell::new(offset),
        };

        Ok(Box::new(reader))
    }
}

impl<'a> wasm::structures::Memory<WITMemoryView<'a>> for WITMemory {
    fn view(&self) -> WITMemoryView<'a> {
        let LocalMemory { base, .. } = unsafe { *self.0.vm_local_memory() };
        let length = self.0.size().bytes().0 / std::mem::size_of::<u8>();

        unsafe { WITMemoryView(MemoryView::new(base as _, length as u32)) }
    }
}
