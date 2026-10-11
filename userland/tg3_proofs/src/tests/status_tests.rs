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

//! The status block the part writes back: the two indexes at tg3_hw_status's
//! offsets for a little-endian host, the link-change flag taken once and
//! acknowledged as tg3_poll_link does, and a clear that stops at its size.

use crate::constants::layout::STATUS_AT;
use crate::constants::ring::HW_STATUS_SIZE;

use super::memory::Memory;

#[test]
fn indexes_are_read_where_the_part_writes_them() {
    let mut m = Memory::new();
    let s = m.status();
    m.put32(STATUS_AT + 16, 0x0123_0456);
    assert_eq!((s.rx_producer(), s.tx_consumer()), (0x0456, 0x0123));
}

#[test]
fn a_link_change_is_taken_once_and_acknowledged() {
    let mut m = Memory::new();
    let s = m.status();
    m.put32(STATUS_AT, 0x0000_0002);
    assert!(s.take_link_change());
    assert_eq!(m.get32(STATUS_AT), 0x0000_0001);
    assert!(!s.take_link_change());
}

#[test]
fn clear_stops_at_the_block() {
    let mut m = Memory::new();
    let s = m.status();
    for at in (0..HW_STATUS_SIZE + 8).step_by(4) {
        m.put32(STATUS_AT + at, !0);
    }
    s.clear();
    assert!((0..HW_STATUS_SIZE).step_by(4).all(|at| m.get32(STATUS_AT + at) == 0));
    assert_eq!(m.get32(STATUS_AT + HW_STATUS_SIZE), !0);
}
