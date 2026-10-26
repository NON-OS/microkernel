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

//! Only a tier that has answered on NONOS is ever a default; an unproven
//! one is taken only when it is named.

use super::GIB;
use crate::default_tier::proven::PROVEN;
use crate::default_tier::{proven, resolve, resolve_in, Source, WEIGHTS};
use crate::need::Room;

/* No default is ever a tier that has not answered on NONOS, on any machine. */
#[test]
fn a_default_is_always_a_proven_tier() {
    for room in [Room::Disk, Room::Memory] {
        for total in (0..=512u64).map(|g| Some(g * GIB / 2)).chain([None]) {
            let (t, why) = resolve(b"", total, room);
            assert!(proven(t), "{t} as {why:?} for {total:?} {room:?}");
        }
    }
    assert!(!proven("qwen3-30b-a3b"), "qwen3moe has not answered on NONOS");
    assert!(PROVEN.iter().all(|(t, _)| WEIGHTS.iter().any(|w| w.0 == *t)), "a mark names no tier");
}

/* An unproven tier can be chosen, but never wins as the largest that fits. */
#[test]
fn an_unproven_tier_is_chosen_only_by_name() {
    static TWO: [(&str, u64); 2] = [("small", 491_400_032), ("qwen3-30b-a3b", 18_556_685_824)];
    let only_small = |t: &str| t == "small";
    let big = Some(64 * GIB);
    assert_eq!(resolve_in(&TWO, only_small, b"", big, Room::Disk), ("small", Source::Largest));
    assert_eq!(
        resolve_in(&TWO, |_| true, b"", big, Room::Disk),
        ("qwen3-30b-a3b", Source::Largest)
    );
    assert_eq!(
        resolve_in(&TWO, only_small, b"qwen3-30b-a3b", big, Room::Disk),
        ("qwen3-30b-a3b", Source::Chosen)
    );
    static UNPROVEN_FIRST: [(&str, u64); 2] = [("qwen3-30b-a3b", 1), ("small", 491_400_032)];
    assert_eq!(
        resolve_in(&UNPROVEN_FIRST, only_small, b"", Some(1), Room::Disk),
        ("small", Source::Smallest)
    );
    assert_eq!(resolve(b"qwen3-30b-a3b", Some(GIB), Room::Disk), ("qwen3-30b-a3b", Source::Chosen));
}
