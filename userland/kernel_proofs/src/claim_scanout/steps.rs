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

//! One atomic step of a path in the takeover model (L79): one hold of a
//! lock, or one act with none, as the kernel takes it; false when the path
//! cannot move. The world is held to its invariants after every step.

use super::knob::Knob;
use super::world_state::World;

pub(super) fn step(w: &mut World, p: usize, k: Knob) -> bool {
    let held = w.held;
    let moved = match p {
        0 | 1 => super::taker::taker(w, p, k),
        2 => super::holder::holder(w, k),
        3 => super::sibling::sibling(w, k),
        4 => super::exit::exit(w),
        5 => super::drawer::drawer(w, k),
        _ => super::exit::orphan(w),
    };
    // HoldKept: the entry's going no longer clears the hold (released).
    w.held |= held && k == Knob::HoldKept;
    if moved {
        w.check();
    }
    moved
}
