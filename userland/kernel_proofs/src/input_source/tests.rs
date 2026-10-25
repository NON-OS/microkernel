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

//! The stamp and the poster table, as MkInputEventPost runs them.

use super::source::{index_of, stamp, KERNEL, SLOTS, SOURCE_BITS, UNREGISTERED};

#[test]
fn a_posters_own_high_byte_is_always_overwritten() {
    for flags in 0..=u16::MAX {
        for source in [KERNEL, 1, 2, 0x1F, 0x20, 0x3E, UNREGISTERED] {
            let out = stamp(flags, source);
            assert_eq!((out & SOURCE_BITS) >> 8, u16::from(source), "{flags:#06x} by {source}");
            assert_eq!(out & !SOURCE_BITS, flags & !SOURCE_BITS, "modifiers and bond marks kept");
        }
    }
}

#[test]
fn a_poster_keeps_one_small_index_that_is_not_its_pid() {
    let mut t = [0u32; SLOTS];
    let live = |_: u32| true;
    assert_eq!(index_of(&mut t, 4096, live), 1, "the first poster");
    assert_eq!(index_of(&mut t, 77, live), 2);
    assert_eq!(index_of(&mut t, 4096, live), 1, "the same process, the same source");
    assert_ne!(u32::from(index_of(&mut t, 2, live)), 2, "an index, never the pid");
}

#[test]
fn a_dead_posters_slot_is_reused_and_a_full_table_says_so() {
    let mut t = [0u32; SLOTS];
    for pid in 1..=SLOTS as u32 {
        assert_eq!(u32::from(index_of(&mut t, 1000 + pid, |_| true)), pid);
    }
    assert_eq!(index_of(&mut t, 5000, |_| true), UNREGISTERED, "every slot live");
    let dead = 1000 + 9;
    assert_eq!(index_of(&mut t, 5000, |p| p != dead), 9, "the dead one's slot");
    assert_eq!(index_of(&mut t, 0, |_| true), UNREGISTERED, "no pid 0 poster");
}
