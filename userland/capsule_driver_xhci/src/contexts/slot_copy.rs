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

//! The Slot Context of an input context for Configure Endpoint (xHCI 1.2
//! section 4.6.6 and 6.2.2.2), copied from the device's output context as
//! Linux's `xhci_slot_copy` does, so what the controller already holds (root
//! port, speed, route, interrupter target, exit latency) is handed back
//! unchanged. Only Context Entries is raised: it names the last valid
//! endpoint context, and lowering it would drop an endpoint configured
//! earlier, as a second interface of a composite device would.

use crate::dma::DmaRegion;

const SLOT_DWORDS: usize = 4;
/// The Slot Context's index in an input context (after the Input Control
/// Context) and in an output device context.
const INPUT_SLOT: usize = 1;
const OUTPUT_SLOT: usize = 0;
const CONTEXT_ENTRIES_SHIFT: u32 = 27;
const CONTEXT_ENTRIES_MASK: u32 = 0x1F << CONTEXT_ENTRIES_SHIFT;

/// Copy the output Slot Context into `input` with Context Entries at least
/// `last_dci`.
pub fn copy_slot_context(input: &DmaRegion, output: &DmaRegion, context_size: u8, last_dci: u8) {
    for dw in 0..SLOT_DWORDS {
        let mut v = read_dw(output, context_size, OUTPUT_SLOT, dw);
        if dw == 0 {
            v = with_entries(v, last_dci);
        }
        write_dw(input, context_size, INPUT_SLOT, dw, v);
    }
}

/// Slot Context dword 0 with Context Entries raised to at least `last_dci`.
pub fn with_entries(dw0: u32, last_dci: u8) -> u32 {
    let entries = ((dw0 & CONTEXT_ENTRIES_MASK) >> CONTEXT_ENTRIES_SHIFT).max(last_dci as u32);
    (dw0 & !CONTEXT_ENTRIES_MASK) | ((entries & 0x1F) << CONTEXT_ENTRIES_SHIFT)
}

pub(super) fn read_dw(region: &DmaRegion, context_size: u8, context: usize, dword: usize) -> u32 {
    let byte = context * context_size as usize + dword * core::mem::size_of::<u32>();
    /*
     * SAFETY: every context region holds at least 32 contexts of
     * `context_size` bytes and `context` is at most 32.
     */
    unsafe { core::ptr::read_volatile(region.as_mut_ptr::<u8>().add(byte) as *const u32) }
}

pub(super) fn write_dw(
    region: &DmaRegion,
    context_size: u8,
    context: usize,
    dword: usize,
    value: u32,
) {
    let byte = context * context_size as usize + dword * core::mem::size_of::<u32>();
    /*
     * SAFETY: as for `read_dw`.
     */
    unsafe {
        core::ptr::write_volatile(region.as_mut_ptr::<u8>().add(byte) as *mut u32, value);
    }
}
