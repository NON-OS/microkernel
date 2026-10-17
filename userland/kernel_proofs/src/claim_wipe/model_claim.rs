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

//! claim.rs `claim`, a step a move: enter.rs's own step of the wipes, the
//! entry (model_enter, which asks the kept bound; undo check 15 reads it at
//! step 1, before the hold), then the steps that touch the function before
//! `admit`: the attach, the move to D0, the BARs read back, no-snoop off and
//! Bus Master off. A refusal after the entry was made stops the function and
//! withdraws it (steps 9 and 10).

use super::bits_rule::{pmcsr_to, NO_SNOOP_EN, PM_STATE};
use super::model_admit::admit;
use super::model_attempt::attempt_over;
use super::model_enter::{enter, ours, withdraw, KEEP_CAP};
use super::model_stepper::Stepper;
use super::phase_rule::MS;
use super::world::World;
use super::world_knob::Knob;
use super::world_thread::Thread;

pub(super) fn claim(w: &mut World, p: usize, t: &mut Thread) {
    let c = w.dev.caps;
    t.step += 1;
    match t.step - 1 {
        // libc asks again only for a live process.
        0 if w.procs[p].dead => attempt_over(w, p, t),
        0 => t.stepper = Some(Stepper::new(true)),
        1 if w.knob == Knob::CapEarly => w.procs[p].over_cap = w.kept_bytes() >= KEEP_CAP,
        2 => enter(w, p, t),
        // confine::attach refused, or a BAR that would not keep (L62).
        3 | 5 if !c.untranslated && w.chance(40) => t.step = 9,
        // power.rs `power_on_device`, and its settling spin.
        4 if c.pm => {
            let pm = w.pmcsr();
            if pm & PM_STATE != 0 {
                w.set_pmcsr(false, pmcsr_to(pm, 0));
                t.at = w.now + 10 * MS;
            }
        }
        5 => w.touch(false),
        6 if c.pcie => {
            let d = w.devctl();
            w.set_devctl(false, d & !NO_SNOOP_EN);
        }
        7 if !c.untranslated => w.stop_bme(),
        8 => admit(w, p, t),
        9 if ours(w, p, t.epoch) => w.stop_bme(),
        10 => withdraw(w, p, t),
        _ => {}
    }
}
