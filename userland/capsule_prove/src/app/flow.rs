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

//! One step a tick, so the window shows each before the next runs. A refusal
//! ends the run and drops everything the steps had made; nothing after it is
//! asked of the kernel.

use super::state::{Line, Mark, Outcome, Stage, Step, Work};
use super::{device, proving, registry};

fn run(step: Step, w: &mut Work) -> Outcome {
    match step {
        Step::Registry => registry::registry(w),
        Step::Slots => device::boot_slots(w),
        Step::Secret => device::secret(w),
        Step::Primed => Ok((
            "Proving: several minutes on one core; the window waits until it is done".into(),
            Some(Step::Prove),
        )),
        Step::Prove => proving::proof(w),
        Step::Verify => proving::check(w),
        Step::Write => proving::save(w),
    }
}

/// Take `step`, log its line, and say where the run stands after it.
pub fn advance(step: Step, w: &mut Work, log: &mut alloc::vec::Vec<Line>) -> Stage {
    match run(step, w) {
        Ok((text, Some(next))) => {
            let mark = if step == Step::Primed { Mark::Note } else { Mark::Ok };
            log.push(Line { mark, text });
            Stage::Running(next)
        }
        Ok((text, None)) => {
            log.push(Line { mark: Mark::Ok, text });
            *w = Work::default();
            Stage::Done
        }
        Err(text) => {
            log.push(Line { mark: Mark::No, text });
            if let Some(mut witness) = w.witness.take() {
                crate::assemble::wipe(&mut witness);
            }
            *w = Work::default();
            Stage::Refused
        }
    }
}
