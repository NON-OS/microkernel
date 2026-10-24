// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! The input context that gives a configured bulk endpoint streams: the
//! endpoint dropped and added again in one Configure Endpoint, with
//! MaxPStreams, LSA (a linear array) and the stream context array in place
//! of its ring. Its type, burst and packet size are kept from the output
//! context, as OP_CONFIGURE_BULK set them.

use super::streams::StreamPipe;
use crate::contexts::copy_slot_context;
use crate::dma::DmaRegion;

const LSA: u32 = 1 << 15;
const AVERAGE_TRB: u32 = 3 * 1024;

fn at(region: &DmaRegion, context_size: u8, context: usize, dword: usize) -> *mut u32 {
    let byte = context * context_size as usize + dword * 4;
    // SAFETY: every context region holds 33 contexts and `context` is at
    // most 32; the pointer is only read or written once, at once.
    unsafe { region.as_mut_ptr::<u8>().add(byte) as *mut u32 }
}

pub fn write_stream_input(
    input: &DmaRegion,
    output: &DmaRegion,
    context_size: u8,
    dci: u8,
    pipe: &StreamPipe,
) {
    input.zero();
    let w = |context: usize, dword: usize, v: u32| {
        // SAFETY: as `at` says.
        unsafe { at(input, context_size, context, dword).write_volatile(v) }
    };
    w(0, 0, 1 << dci);
    w(0, 1, 1 | 1 << dci);
    copy_slot_context(input, output, context_size, dci);
    // SAFETY: as `at` says.
    let kept = unsafe { at(output, context_size, dci as usize, 1).read_volatile() };
    let ep = dci as usize + 1;
    w(ep, 0, (pipe.max_pstreams() as u32) << 10 | LSA);
    w(ep, 1, kept);
    w(ep, 2, pipe.array.phys() as u32);
    w(ep, 3, (pipe.array.phys() >> 32) as u32);
    w(ep, 4, AVERAGE_TRB);
}
