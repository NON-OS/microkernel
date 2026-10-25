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

//! One actor's move: a stage of the `step_all` call a thread or a tick is
//! inside, else the next step of the thread's path.

use super::model_attempt::begin;
use super::model_claim::claim;
use super::model_exit::exit;
use super::model_hold::hold;
use super::model_leave::give;
use super::model_step::stage;
use super::model_stepper::Stepper;
use super::phase_rule::MS;
use super::world::World;
use super::world_thread::Path;

/// A thread's or a tick's move: a stage of the `step_all` call it is in,
/// else its path's next step. A tick that fires begins a call that only
/// tries the table. A main thread done in user code exits.
pub(super) fn move_thread(w: &mut World, a: usize) {
    let (mut t, p) = (w.threads[a], a / 3);
    let was = t.path;
    if a >= 9 && t.stepper.is_none() {
        t.stepper = Some(Stepper::new(false));
    }
    if let Some(mut s) = t.stepper {
        let more = stage(w, &mut s, a);
        let wait = match more {
            Some(d) => d,
            None if a >= 9 => tick_gap(w),
            None => 0,
        };
        (t.stepper, t.at) = (more.map(|_| s), w.now + wait);
    } else {
        match t.path {
            Path::Idle => begin(w, p, &mut t),
            Path::Claim => claim(w, p, &mut t),
            Path::Hold => hold(w, p, &mut t),
            Path::Give => give(w, p, &mut t),
            Path::Exit => exit(w, p, &mut t),
            Path::Off | Path::Done => {}
        }
    }
    w.threads[a] = t;
    if a < 9 && a.is_multiple_of(3) && t.path == Path::Done && was != Path::Done {
        w.exit_now(p);
    }
}

/// The next tick: 0 to 40 ms on, or up to 200 ms with its CPU in kernel code.
fn tick_gap(w: &mut World) -> u64 {
    MS * if w.chance(25) { w.draw(201) } else { w.draw(41) }
}
