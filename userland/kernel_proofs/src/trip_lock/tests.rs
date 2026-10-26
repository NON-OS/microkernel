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

//! The model's table (L81b plan, section 7): the lock as built is clean on
//! one CPU and two, and every knob that weakens it is caught, bad or stuck.
//! NoNextSweep alone is clean by design: staleness backs it, and the knob
//! that also removes staleness is stuck. EndedOnly breaks only on SMP.

use super::search::search;
use super::world::Knob::{self, *};

/// (knob, one CPU, bad, stuck, worlds, taken back)
const TABLE: [(Knob, bool, u16, bool, usize, bool); 26] = [
    (Built, true, 0x00, false, 82472, true),
    (Built, false, 0x00, false, 349114, true),
    (NoSweep, true, 0x00, true, 54852, true),
    (NoSweep, false, 0x00, true, 215552, true),
    (EndedOnly, true, 0x00, false, 82472, true),
    (EndedOnly, false, 0x1b, false, 1148134, true),
    (OffCpuOnly, true, 0x1b, false, 293593, true),
    (OffCpuOnly, false, 0x1f, false, 3649915, true),
    (NoNextSweep, true, 0x00, false, 91178, true),
    (NoNextSweep, false, 0x00, false, 371732, true),
    (NoForget, true, 0x20, true, 99308, true),
    (NoForget, false, 0x20, true, 394640, true),
    (NeverStale, true, 0x00, true, 49376, true),
    (NeverStale, false, 0x00, true, 242990, true),
    (NoNestedRead, true, 0x10, false, 124369, true),
    (NoNestedRead, false, 0x10, false, 508403, true),
    (NoWakeList, true, 0x04, false, 99266, true),
    (NoWakeList, false, 0x04, false, 485876, true),
    (NoLetGo, true, 0x02, false, 86658, true),
    (NoLetGo, false, 0x02, false, 410136, true),
    (NoPutRight, true, 0x02, false, 90632, true),
    (NoPutRight, false, 0x02, false, 403746, true),
    (ForgetKeepsWaiting, true, 0x20, false, 90840, true),
    (ForgetKeepsWaiting, false, 0x20, false, 368726, true),
    (NoNextSweepNeverStale, true, 0x00, true, 49096, true),
    (NoNextSweepNeverStale, false, 0x00, true, 236486, true),
];

#[test]
fn the_lock_as_built_is_clean_and_a_killed_sleepers_lock_comes_back() {
    for up in [true, false] {
        assert_eq!(search(Built, up), (0x00, false, if up { 82472 } else { 349114 }, true));
    }
}

#[test]
fn every_weakened_lock_gives_its_row_of_the_table() {
    for (knob, up, bad, stuck, worlds, took) in TABLE {
        assert_eq!(search(knob, up), (bad, stuck, worlds, took), "{knob:?} one CPU {up}");
    }
}
