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

//! The cases the wipe's rule tables share: every phase, method, marker and
//! start look. A phase added to the rule and not listed here stops the
//! build, so no table can quietly skip it.

use super::marker_rule::Marker;
use super::method_rule::Method;
use super::phase_rule::Phase;
use super::start_rule::Look;

pub(super) const METHODS: [Method; 3] = [Method::Flr, Method::AfFlr, Method::PmCycle];
pub(super) const MARKERS: [Marker; 2] = [Marker::IntxDisable, Marker::MemorySpace];

/// Every phase, each timed one begun at `at`, in `index` order.
pub(super) const fn phases(at: u64) -> [Phase; 8] {
    [
        Phase::Held,
        Phase::Owed { since: at },
        Phase::Flr { at },
        Phase::D3 { at },
        Phase::D0 { at },
        Phase::Recheck,
        Phase::Kept,
        Phase::Lost { at },
    ]
}

/// No catch-all arm: a new phase fails to build here until it is listed.
pub(super) const fn index(p: Phase) -> usize {
    match p {
        Phase::Held => 0,
        Phase::Owed { .. } => 1,
        Phase::Flr { .. } => 2,
        Phase::D3 { .. } => 3,
        Phase::D0 { .. } => 4,
        Phase::Recheck => 5,
        Phase::Kept => 6,
        Phase::Lost { .. } => 7,
    }
}

/// Every method a function may offer, none among them, with every look.
pub(super) fn looks() -> Vec<(Option<Method>, Look)> {
    let mut all = Vec::new();
    for m in [None].into_iter().chain(METHODS.map(Some)) {
        for marker in [None].into_iter().chain(MARKERS.map(Some)) {
            for (in_d0, tp) in [(false, false), (false, true), (true, false), (true, true)] {
                all.push((m, Look { in_d0, tp, marker }));
            }
        }
    }
    all
}

#[test]
fn every_phase_is_listed_once_in_its_place() {
    assert!(phases(41).iter().enumerate().all(|(i, &p)| index(p) == i));
    assert_eq!(looks().len(), 4 * 3 * 4);
}
