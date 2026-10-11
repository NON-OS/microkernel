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

//! The input context that adds one isochronous OUT endpoint (xHCI 1.2
//! sections 4.14.2 and 6.2.3).

use super::endpoint::IsochEndpoint;
use crate::contexts::copy_slot_context;
use crate::dma::DmaRegion;

const EP_TYPE_ISOCH_OUT: u32 = 1;

/// CErr is 0: an isoch endpoint does not retry, a late packet is dropped.
pub fn write_isoch_input(
    region: &DmaRegion,
    output: &DmaRegion,
    context_size: u8,
    ep: IsochEndpoint,
) {
    region.zero();
    write_dw(region, context_size, 0, 1, 1 | (1u32 << ep.dci));
    copy_slot_context(region, output, context_size, ep.dci);
    let esit = ep.esit_payload();
    let at = ep.dci as usize + 1;
    let dw0 = ((ep.mult as u32 & 0x3) << 8) | ((ep.interval as u32) << 16) | ((esit >> 16) << 24);
    let dw1 =
        (EP_TYPE_ISOCH_OUT << 3) | ((ep.max_burst as u32) << 8) | ((ep.max_packet as u32) << 16);
    write_dw(region, context_size, at, 0, dw0);
    write_dw(region, context_size, at, 1, dw1);
    write_dw(region, context_size, at, 2, (ep.ring_phys as u32) | 1);
    write_dw(region, context_size, at, 3, (ep.ring_phys >> 32) as u32);
    // Average TRB length: one TRB per TD, so a whole interval's payload.
    write_dw(region, context_size, at, 4, (esit & 0xFFFF) | ((esit & 0xFFFF) << 16));
}

fn write_dw(region: &DmaRegion, context_size: u8, context: usize, dword: usize, value: u32) {
    let byte = context * context_size as usize + dword * core::mem::size_of::<u32>();
    /*
     * SAFETY: the region holds the whole input context, 33 contexts of
     * `context_size` bytes, and `context` is at most 32.
     */
    unsafe {
        core::ptr::write_volatile(region.as_mut_ptr::<u8>().add(byte) as *mut u32, value);
    }
}
