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

//! Every return-ring entry is the part's to write. Whatever three words it
//! leaves, an entry this driver acts on names a slot of the standard ring
//! and a length that fits the reply, over a long run of random words.

use crate::constants::desc::{RXD_ERR_MASK, RXD_OPAQUE_RING_MASK, RXD_OPAQUE_RING_STD};
use crate::constants::ring::RX_STD_RING_SIZE;
use crate::constants::{MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
use crate::queue::rx_parse::{parse, Entry};

const STD: u32 = RXD_OPAQUE_RING_STD;

#[test]
fn hostile_words_never_escape_their_bounds() {
    let mut x = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..200_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let (opaque, idx_len, err) = (x as u32, (x >> 16) as u32, (x >> 32) as u32 & 0x01FF_0000);
        match parse(opaque, idx_len, err) {
            Entry::Foreign => assert!(
                opaque & RXD_OPAQUE_RING_MASK != STD || opaque & 0xFFFF >= RX_STD_RING_SIZE as u32
            ),
            Entry::Drop(i) => assert!((i as usize) < RX_STD_RING_SIZE),
            Entry::Frame(i, n) => {
                assert!((i as usize) < RX_STD_RING_SIZE && err & RXD_ERR_MASK == 0);
                assert!((MIN_ETHERNET_FRAME..=MAX_ETHERNET_FRAME).contains(&(n as usize)));
            }
        }
    }
}
