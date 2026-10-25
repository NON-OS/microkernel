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

//! The contender's steps (L77e): `lock_yielding`'s try, and its takeover by
//! the kernel's rule, or by a weakened one under a knob; then it lets go.

use super::transport_rule::takes_over;
use super::world::{Knob, World, H};

pub fn contender(w: &mut World, k: Knob) -> bool {
    let on_cpu = w.h == H::Running;
    let takes = match k {
        Knob::NoTakeover => false,
        Knob::DeadOnly => !w.alive,
        Knob::OffCpuOnly => !on_cpu,
        _ => takes_over(1, !w.alive, on_cpu),
    };
    match w.c {
        0 if !w.locked => (w.locked, w.row_h, w.c) = (true, false, 1),
        0 if w.row_h && takes => w.locked = false,
        1 => (w.locked, w.c) = (false, 2),
        _ => return false,
    }
    true
}
