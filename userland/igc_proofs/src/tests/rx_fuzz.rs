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

//! The receive write-back is written by the part, so status_error and length
//! are hostile. Only a descriptor with DD and EOP, without RXE, and with
//! 0 < length <= 1514 yields a frame, and every consumed slot goes back in
//! read format with its own buffer address.

use crate::constants::queue::RX_DESC_COUNT;
use crate::constants::rx_bits::{RXDEXT_STATERR_RXE, RXD_STAT_DD, RXD_STAT_EOP};
use crate::constants::MAX_ETHERNET_FRAME;
use crate::queue::layout::RxDesc;

use super::memory::Memory;

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn consume_passes_only_whole_bounded_error_free_frames() {
    let mut mem = Memory::new();
    for seed in 1..200_000u64 {
        let mut s = seed;
        let head = (xorshift(&mut s) % RX_DESC_COUNT as u64) as u16;
        let mut staterr = xorshift(&mut s) as u32;
        if seed % 2 == 0 {
            staterr |= RXD_STAT_DD | RXD_STAT_EOP;
        }
        let length = (xorshift(&mut s) % 3000) as u16;
        mem.rx[head as usize] = RxDesc { buffer_addr: !0, staterr, length, vlan: 0xFFFF };
        let mut rx = mem.rx_ring();
        rx.head = head;
        let got = rx.consume();
        if staterr & RXD_STAT_DD == 0 {
            assert_eq!(got, None, "a slot without DD is not the driver's");
            assert_eq!(rx.head, head);
            continue;
        }
        let (idx, len) = got.expect("DD set: the slot is consumed");
        assert_eq!(idx, head);
        assert_eq!(rx.head, (head + 1) % RX_DESC_COUNT as u16);
        let whole = staterr & RXD_STAT_EOP != 0 && staterr & RXDEXT_STATERR_RXE == 0;
        let fits = length != 0 && length as usize <= MAX_ETHERNET_FRAME;
        assert_eq!(len, if whole && fits { length } else { 0 });
        let d = mem.rx[idx as usize];
        assert_eq!(d.buffer_addr, rx.buffer_phys(idx), "read format, own buffer");
        assert_eq!((d.staterr, d.length, d.vlan), (0, 0, 0), "hdr_addr is zero");
    }
}
