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
use crate::dma::doorbell_barrier;
use crate::regs::mmio_write32;
pub fn ring_doorbell(doorbell_base: u64, slot: u8, target: u8) {
    let addr = doorbell_base + (slot as u64) * 4;
    // The controller DMA-reads the ring's TRBs only after it sees the doorbell.
    // Order every prior TRB store ahead of this write so it never fetches a
    // stale or half-written TRB.
    doorbell_barrier();
    // SAFETY: `doorbell_base` heads the doorbell page `read_layout` checked lies inside the mapped
    // window, and a u8 slot reaches at most 1020 bytes into it.
    unsafe { mmio_write32(addr, target as u32) };
}

/// The doorbell for stream `stream` of endpoint `target`: DB Stream ID in
/// bits 31:16 (xHCI 1.2, 5.6, as recalled, unverified).
pub fn ring_doorbell_stream(doorbell_base: u64, slot: u8, target: u8, stream: u16) {
    let addr = doorbell_base + (slot as u64) * 4;
    doorbell_barrier();
    // SAFETY: as in `ring_doorbell`.
    unsafe { mmio_write32(addr, target as u32 | (stream as u32) << 16) };
}
