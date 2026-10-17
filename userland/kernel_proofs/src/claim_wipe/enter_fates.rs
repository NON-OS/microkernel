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

//! What the device in the enter model does with its reset (L78), and every
//! start the model is run from: each fate, with A's release, its exit or
//! both, and B, A2 or both claiming.

use super::enter_world::{start, World};
use super::phase_rule::{FLR_WAIT, MS, RETRY_WINDOW};

/// The device: an FLR offered, a reset that reaches its registers, how long
/// after one it answers, and memory a reset leaves (a display function's,
/// recalled, unverified).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(super) struct Fate {
    pub flr: bool,
    pub obeys: bool,
    pub back: u64,
    pub display: bool,
}

const fn fate(flr: bool, obeys: bool, back: u64, display: bool) -> Fate {
    Fate { flr, obeys, back, display }
}

/// Clean; Clean but a display function's memory kept; no way to reset it;
/// a reset that leaves its marker; back only after its deadline; never back.
pub(super) const FATES: [(&str, Fate); 6] = [
    ("clean", fate(true, true, FLR_WAIT, false)),
    ("display", fate(true, true, FLR_WAIT, true)),
    ("no way", fate(false, false, FLR_WAIT, false)),
    ("ignores", fate(true, false, FLR_WAIT, false)),
    ("late", fate(true, true, FLR_WAIT + RETRY_WINDOW + MS, false)),
    ("never", fate(true, true, u64::MAX, false)),
];

/// The fates whose end leaves the function for A's own service only.
pub(super) const fn kept(f: Fate) -> bool {
    f.display || !f.flr || !f.obeys
}

/// Every start for one fate: A's release, exit or both, against B, A2 or
/// both.
pub(super) fn starts(f: Fate) -> impl Iterator<Item = World> {
    let sides = [(true, false), (false, true), (true, true)];
    let claimants = [(true, false), (false, true), (true, true)];
    sides
        .into_iter()
        .flat_map(move |(r, x)| claimants.into_iter().map(move |(b, a2)| start(f, r, x, b, a2)))
}
