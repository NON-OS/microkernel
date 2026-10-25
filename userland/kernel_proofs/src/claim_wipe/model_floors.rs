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

//! The floors the seeded sweeps must reach (world_cases), and the sweep as
//! built that every seed of which holds every invariant.

use super::method_rule::Method;
use super::model_sweep::sweep;
use super::world_cases::WITHDRAWN;
use super::world_cases::{A2_KEPT, BUSY, B_CLEAN, CAP, CLEAN, CONTEND, KILLED, LET_GO_FIRST};
use super::world_cases::{LOST, LOST_ADMIT, RETRIED, SEEN, SETTLE_FIRST, TP_STAY, UNRESET};
use super::world_knob::Knob;

/// Reached whatever the method: a claimant killed, a claim withdrawn, one
/// refused over the kept bound, kept runs settled before and after the
/// registry let go, A2 let in over a kept function.
pub(super) const EVERY: [(usize, u32); 6] = [
    (KILLED, 700),
    (WITHDRAWN, 250),
    (CAP, 800),
    (SETTLE_FIRST, 900),
    (LET_GO_FIRST, 900),
    (A2_KEPT, 700),
];

/// Reached where a reset is written: B let in after a clean one, a lost
/// function asked again and someone let in after, a start held off by
/// pending transactions, a tick finding the table held, a step finding the
/// record being stepped, libc asking again, and each end.
pub(super) const RESETS: [(usize, u32); 9] = [
    (B_CLEAN, 1_200),
    (LOST_ADMIT, 200),
    (TP_STAY, 20_000),
    (CONTEND, 3_000),
    (BUSY, 4_000),
    (RETRIED, 40_000),
    (CLEAN, 4_000),
    (UNRESET, 800),
    (LOST, 1_200),
];

pub(super) fn holds(m: Option<Method>, more: &[(usize, u32)]) -> [u32; SEEN] {
    let s = sweep(m, Knob::Built, 5_000, 0);
    assert_eq!(s.broke, 0, "{m:?}: (seed, invariants broken, the first) {:?}", s.first);
    for &(case, floor) in EVERY.iter().chain(more) {
        assert!(s.seen[case] >= floor, "{m:?}: case {case} under {floor}: {:?}", s.seen);
    }
    s.seen
}
