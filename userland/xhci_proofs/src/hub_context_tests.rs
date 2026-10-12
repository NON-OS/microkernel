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
//! The input contexts the hub ops hand the controller, built on DMA as the
//! handlers build them: Address Device for a routed device, and Configure
//! Endpoint marking a hub with only the Slot Context added.

use crate::contexts::{write_hub_input, write_routed};
use crate::dma::{DmaPool, DmaRegion};
use crate::slots::SlotResources;

const CSZ: u8 = 32;

fn dw(region: &DmaRegion, context: usize, dword: usize) -> u32 {
    let at = context * CSZ as usize + dword * 4;
    // SAFETY: a live context region of at least 33 contexts.
    unsafe { core::ptr::read_volatile(region.as_mut_ptr::<u8>().add(at) as *const u32) }
}

#[test]
fn a_routed_address_carries_route_speed_root_port_and_tt() {
    let pool = DmaPool::new(1, 1);
    let res = SlotResources::allocate(&pool, CSZ, 6, 2, 1, false).expect("slot");
    write_routed(&res.input_context, CSZ, 0x31, Some((4, 3)));
    assert_eq!(dw(&res.input_context, 0, 1), 0x3, "A0 and A1: slot and EP0");
    let slot0 = dw(&res.input_context, 1, 0);
    assert_eq!((slot0 & 0xF_FFFF, slot0 >> 20 & 0xF, slot0 >> 27), (0x31, 1, 1));
    assert_eq!(dw(&res.input_context, 1, 1) >> 16 & 0xFF, 2, "root port 2");
    assert_eq!(dw(&res.input_context, 1, 2) & 0xFFFF, 4 | 3 << 8, "TT hub slot 4, port 3");
    assert!(!res.hub, "a device is no hub until configured as one");
}

#[test]
fn a_high_speed_device_below_a_hub_names_no_tt() {
    let pool = DmaPool::new(1, 1);
    let res = SlotResources::allocate(&pool, CSZ, 7, 1, 3, false).expect("slot");
    write_routed(&res.input_context, CSZ, 0x2, None);
    assert_eq!(dw(&res.input_context, 1, 0) & 0xF_FFFF, 0x2);
    assert_eq!(dw(&res.input_context, 1, 2) & 0xFFFF, 0);
}

#[test]
fn configuring_a_hub_adds_only_the_slot_and_keeps_its_context() {
    let pool = DmaPool::new(1, 1);
    let res = SlotResources::allocate(&pool, CSZ, 3, 1, 3, false).expect("slot");
    // The device's output Slot Context as the controller left it: route
    // 0, high speed, 3 entries, root port 1, interrupter 0, address 5.
    let words = [3 << 20 | 3 << 27, 1 << 16, 0, 5];
    for (i, w) in words.into_iter().enumerate() {
        // SAFETY: the output context region holds the Slot Context first.
        unsafe { (res.output_context.as_mut_ptr::<u32>()).add(i).write_volatile(w) };
    }
    write_hub_input(&res.input_context, &res.output_context, CSZ, (4, 1, false));
    assert_eq!((dw(&res.input_context, 0, 0), dw(&res.input_context, 0, 1)), (0, 1), "A0 only");
    let s = [0, 1, 2, 3].map(|d| dw(&res.input_context, 1, d));
    assert_eq!(s[0], 3 << 20 | 3 << 27 | 1 << 26, "Hub set; entries kept, so no endpoint drops");
    assert_eq!(s[1], 1 << 16 | 4 << 24, "4 ports on root port 1");
    assert_eq!(s[2], 1 << 16, "think time 1");
    assert_eq!(s[3], 5);
    assert_eq!(dw(&res.input_context, 2, 1), 0, "EP0's context is not handed in");
}
