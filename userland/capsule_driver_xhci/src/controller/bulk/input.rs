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
//! The input context that adds a bulk IN and a bulk OUT endpoint to a slot
//! (xHCI 1.2 sections 4.6.6 and 6.2.3).
use crate::dma::DmaRegion;
const SLOT_CTX: usize = 1;
const EP_TYPE_BULK_OUT: u32 = 2;
const EP_TYPE_BULK_IN: u32 = 6;
const CERR: u32 = 3;
/// The average TRB length the specification suggests for bulk endpoints.
const AVERAGE_TRB: u32 = 3 * 1024;
/// One endpoint: its DCI, ring, and largest packet.
#[derive(Clone, Copy)]
pub struct BulkEndpoint {
    pub dci: u8,
    pub ring_phys: u64,
    pub max_packet: u16,
}
pub fn write_bulk_input(
    region: &DmaRegion,
    context_size: u8,
    speed_port: (u8, u8),
    bulk_in: BulkEndpoint,
    bulk_out: BulkEndpoint,
) {
    region.zero();
    let add = 1 | (1u32 << bulk_in.dci) | (1u32 << bulk_out.dci);
    write_dw(region, context_size, 0, 1, add);
    let entries = bulk_in.dci.max(bulk_out.dci) as u32;
    write_dw(region, context_size, SLOT_CTX, 0, ((speed_port.0 as u32) << 20) | (entries << 27));
    write_dw(region, context_size, SLOT_CTX, 1, (speed_port.1 as u32) << 16);
    endpoint(region, context_size, bulk_in, EP_TYPE_BULK_IN);
    endpoint(region, context_size, bulk_out, EP_TYPE_BULK_OUT);
}
fn endpoint(region: &DmaRegion, context_size: u8, ep: BulkEndpoint, ep_type: u32) {
    let at = ep.dci as usize + 1;
    let dw1 = (CERR << 1) | (ep_type << 3) | ((ep.max_packet as u32) << 16);
    write_dw(region, context_size, at, 1, dw1);
    write_dw(region, context_size, at, 2, (ep.ring_phys as u32) | 1);
    write_dw(region, context_size, at, 3, (ep.ring_phys >> 32) as u32);
    write_dw(region, context_size, at, 4, AVERAGE_TRB);
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
