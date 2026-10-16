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
use super::slot::{slot_dwords, Place};
use super::slot_copy::write_dw;
use crate::dma::DmaRegion;
const ADD_SLOT_AND_EP0: u32 = 0x3;
const SLOT_CTX_INDEX: usize = 1;
const EP0_CTX_INDEX: usize = 2;
const EP_TYPE_CONTROL: u32 = 4;
/// The input context of Address Device: the Slot Context of a device at
/// `place`, and EP0 with `max_packet` on the ring at `ep0_ring_phys`.
pub fn write_address_device_input(
    region: &DmaRegion,
    context_size: u8,
    place: &Place,
    max_packet: u16,
    ep0_ring_phys: u64,
) {
    region.zero();
    write_dw(region, context_size, 0, 1, ADD_SLOT_AND_EP0);
    for (dw, value) in slot_dwords(place).into_iter().enumerate() {
        write_dw(region, context_size, SLOT_CTX_INDEX, dw, value);
    }
    write_ep0(region, context_size, max_packet, ep0_ring_phys);
}
fn write_ep0(region: &DmaRegion, context_size: u8, max_packet: u16, ring_phys: u64) {
    let dw1 = (3 << 1) | (EP_TYPE_CONTROL << 3) | ((max_packet as u32) << 16);
    write_dw(region, context_size, EP0_CTX_INDEX, 1, dw1);
    write_dw(region, context_size, EP0_CTX_INDEX, 2, (ring_phys as u32) | 1);
    write_dw(region, context_size, EP0_CTX_INDEX, 3, (ring_phys >> 32) as u32);
    write_dw(region, context_size, EP0_CTX_INDEX, 4, 8);
}
