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

//! The RX ring's wrap and its slot layout.

use crate::constants::queue::{RX_BUFFER_LEN, RX_DESC_COUNT, RX_STATUS_DD, RX_STATUS_EOP};
use crate::queue::layout::RxDesc;
use crate::queue::RxRing;

#[test]
fn the_ring_wraps_after_its_last_slot() {
    let mut ring = [RxDesc::default(); RX_DESC_COUNT];
    let mut rx = RxRing::new(ring.as_mut_ptr() as u64, 0, 0);
    for round in 0..(3 * RX_DESC_COUNT) {
        let at = round % RX_DESC_COUNT;
        ring[at] = RxDesc { length: 60, status: RX_STATUS_DD | RX_STATUS_EOP, ..RxDesc::default() };
        assert_eq!(rx.consume(), Some((at as u16, 60)));
    }
    assert_eq!(rx.consume(), None, "every slot was handed back cleared");
}

#[test]
fn slot_addresses_are_laid_out_by_buffer_len() {
    let rx = RxRing::new(0, 0x10_0000, 0x20_0000);
    for idx in 0..RX_DESC_COUNT as u16 {
        assert_eq!(rx.buffer_va(idx), 0x10_0000 + idx as u64 * RX_BUFFER_LEN as u64);
        assert_eq!(rx.buffer_phys(idx), 0x20_0000 + idx as u64 * RX_BUFFER_LEN as u64);
    }
}
