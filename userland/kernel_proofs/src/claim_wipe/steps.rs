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

//! The scheduler: each move is one actor's, drawn among the threads, the two
//! CPUs' ticks and the registry able to move now; when none can, time runs
//! on to the next one due. A kernel step takes no time, so a `step_all` call
//! acts at its own clock's reading (the `Stall` knob lifts that); user code,
//! the ticks' gaps and the stalls drawn in teardowns and settles take time.

use super::model_movable::{movable, next, quiet, REGISTRY};
use super::model_move::move_thread;
use super::phase_rule::MS;
use super::world::World;

pub(super) fn run(w: &mut World) {
    let mut moves = 0u32;
    while !quiet(w) {
        let (mut can, mut n) = ([0; 12], 0);
        for a in (0..12).filter(|&a| movable(w, a)) {
            (can[n], n) = (a, n + 1);
        }
        if n == 0 {
            w.now = next(w);
            continue;
        }
        match can[w.draw(n as u64) as usize] {
            REGISTRY => w.let_go(),
            a => move_thread(w, a),
        }
        w.check_step();
        moves += 1;
        if moves > 1_000_000 || w.now > 60_000 * MS {
            return w.bad(5, "the seed never settled");
        }
    }
    w.check_end();
}
