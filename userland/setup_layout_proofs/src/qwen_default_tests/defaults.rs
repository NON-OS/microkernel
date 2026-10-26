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

//! The tier setup's Qwen step starts on, and the weights it is chosen by.

use super::{tiers, GIB};
use crate::default_tier::{resolve, Source};
use crate::need::STICK_TIER;

/*
 * The one rule the dock, the Terminal and setup share: the person's choice;
 * else the stick tier when it fits; else the largest that fits; else the
 * smallest, said as not fitting. Never a coder tier no one chose.
 */
#[test]
fn the_default_is_the_choice_then_the_stick_then_the_largest_that_fits() {
    use crate::need::Room;
    assert_eq!(resolve(b"coder-1.5b", Some(64 * GIB), Room::Disk), ("coder-1.5b", Source::Chosen));
    assert_eq!(resolve(b"qwen3-4b\0\0", Some(GIB), Room::Memory), ("qwen3-4b", Source::Chosen));
    for total in [Some(2 * GIB), Some(8 * GIB), Some(128 * GIB), None] {
        assert_eq!(resolve(b"", total, Room::Disk), (STICK_TIER, Source::Stick), "{total:?}");
        assert_eq!(resolve(b"not-a-tier", total, Room::Disk).0, STICK_TIER);
    }
    assert_eq!(resolve(b"", Some(1_800_000_000), Room::Disk), ("small", Source::Largest));
    assert_eq!(resolve(b"", Some(GIB), Room::Disk), ("small", Source::Smallest));
    assert_eq!(resolve(b"", Some(2 * GIB), Room::Memory), ("small", Source::Smallest));
    for total in [GIB, 2 * GIB, 3 * GIB, 64 * GIB] {
        for room in [Room::Disk, Room::Memory] {
            assert!(!resolve(b"", Some(total), room).0.starts_with("coder-"), "{total} {room:?}");
        }
    }
}

/* The weights the default is weighed by are the pins' own sums, smallest first. */
#[test]
fn the_default_weighs_the_pinned_files() {
    let t = tiers();
    let w = crate::default_tier::WEIGHTS;
    assert_eq!(w.len(), t.len());
    for ((word, bytes), (tier, sum)) in w.iter().zip(&t) {
        assert_eq!((word.as_bytes(), *bytes), (*tier, *sum));
    }
}
