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

//! The set's bit order, and the meet an exec takes.

use crate::SyscallSet;

#[test]
fn meet_keeps_only_what_both_allow() {
    let a = SyscallSet::NONE.with(0).with(1).with(600);
    let b = SyscallSet::NONE.with(1).with(2).with(512);
    let m = a.meet(&b);
    for nr in 0..600 {
        assert_eq!(m.allows(nr), nr == 1 || nr >= 512, "{nr}");
    }
    assert_eq!(SyscallSet::ALL.meet(&a), a);
    assert_eq!(a.meet(&SyscallSet::NONE), SyscallSet::NONE);
}

/// LSB first, as the proof and the `ok=` diagnostic read it: bucket b is
/// bit b % 8 of byte b / 8.
#[test]
fn the_bit_order_is_pinned() {
    let b = SyscallSet::NONE.with(231);
    for (i, byte) in b.as_bytes().iter().enumerate() {
        assert_eq!(*byte, if i == 28 { 0x80 } else { 0 }, "byte {i}");
    }
    assert_eq!(SyscallSet::NONE.with(0).as_bytes()[0], 0x01);
    assert_eq!(SyscallSet::NONE.with(512).as_bytes()[64], 0x01);
}
