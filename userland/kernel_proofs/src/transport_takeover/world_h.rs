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

//! The holder's own steps (L77e): `lock_yielding`, then `round_trip`'s rounds,
//! a dead caller returning at once from a round where it is on its CPU
//! (`NoDeadReturn`: it parks still holding its guard, as before L77e).

use super::world::{Knob, World, H};

pub fn holder(w: &mut World, k: Knob) -> bool {
    match w.h {
        H::Out if !w.alive => w.h = H::Gone,
        H::Out if !w.locked => (w.locked, w.row_h, w.holds, w.h) = (true, true, true, H::Running),
        H::Running if !w.alive && k != Knob::NoDeadReturn => {
            w.drop_guard();
            w.h = H::Gone;
        }
        H::Running if !w.alive => w.h = H::Gone,
        H::Running if w.rounds > 0 => (w.rounds, w.h) = (w.rounds - 1, H::Asleep),
        H::Running => {
            w.drop_guard();
            w.h = H::Gone;
        }
        H::Asleep if w.alive => w.h = H::Running,
        _ => return false,
    }
    true
}
