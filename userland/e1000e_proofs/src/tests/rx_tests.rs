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

//! RX against hostile descriptors. The part writes status, errors and length;
//! the real `consume` must hand a length to the copy path only for a
//! completed, error-free end-of-packet frame that fits an Ethernet frame,
//! which fits the slot, so the handler's copy cannot leave the slot.

use super::xorshift;
use crate::constants::queue::{RX_BUFFER_LEN, RX_DESC_COUNT, RX_STATUS_DD, RX_STATUS_EOP};
use crate::constants::MAX_ETHERNET_FRAME;
use crate::queue::layout::RxDesc;
use crate::queue::RxRing;

const _: () = assert!(MAX_ETHERNET_FRAME <= RX_BUFFER_LEN);

#[test]
fn consume_passes_only_complete_error_free_bounded_frames() {
    let mut ring = [RxDesc::default(); RX_DESC_COUNT];
    for seed in 1..200_000u64 {
        let mut s = seed;
        let head = (xorshift(&mut s) % RX_DESC_COUNT as u64) as u16;
        let status = (xorshift(&mut s) & 0xff) as u8;
        let errors = (xorshift(&mut s) & 0xff) as u8;
        let length = (xorshift(&mut s) & 0xffff) as u16;
        ring[head as usize] = RxDesc { length, status, errors, ..RxDesc::default() };
        let mut rx = RxRing::new(ring.as_mut_ptr() as u64, 0, 0);
        rx.head = head;
        match rx.consume() {
            None => {
                assert_eq!(status & RX_STATUS_DD, 0, "only an incomplete descriptor is skipped");
                assert_eq!(rx.head, head, "and it does not advance the ring");
            }
            Some((idx, len)) => {
                assert_ne!(status & RX_STATUS_DD, 0);
                assert_eq!(idx, head, "the slot handed out is the head");
                assert_eq!(rx.head, (head + 1) % RX_DESC_COUNT as u16);
                assert_eq!((ring[idx as usize].status, ring[idx as usize].errors), (0, 0));
                let whole = status & RX_STATUS_EOP != 0 && errors == 0;
                if !whole || length == 0 || length as usize > MAX_ETHERNET_FRAME {
                    assert_eq!(len, 0, "a partial, errored or oversized frame carries no length");
                } else {
                    assert_eq!(len, length);
                }
            }
        }
    }
}
