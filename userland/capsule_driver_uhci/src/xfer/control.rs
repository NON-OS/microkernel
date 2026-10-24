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

//! A control transfer on the control queue (plan.rs for its TDs). A short
//! IN data packet ends the data stage early; STATUS then runs on its own.

use super::chain::{moved, write};
use super::plan::control_plan;
use super::run::{run, Ran};
use crate::env::{Clock, Mem};
use crate::facts::bounds::CONTROL_MS;
use crate::facts::td::TD_LEN;
use crate::sched::layout::{CTRL_QH, CTRL_TD, SETUP_BUF};
use crate::sched::td::Target;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CtrlError {
    Stalled,
    Bus(u32),
    TimedOut,
    /// Longer than the data buffer, or more packets than the chain holds.
    TooLong,
}

/// Run `setup`; an IN request's data lands in the control buffer, an OUT
/// request's is taken from it. Returns the data bytes moved.
pub fn control<M: Mem, C: Clock>(
    m: &M,
    c: &C,
    t: Target,
    mps: u16,
    setup: [u8; 8],
) -> Result<usize, CtrlError> {
    let (steps, n, packets) = control_plan(setup, mps).ok_or(CtrlError::TooLong)?;
    m.write_bytes(SETUP_BUF, &setup);
    write(m, CTRL_TD, t, &steps[..n]);
    match run(m, c, CTRL_QH, CTRL_TD, n, CONTROL_MS) {
        Ran::Ended { upto, short: false } if upto == n - 1 => Ok(moved(m, CTRL_TD, 1, packets)),
        Ran::Ended { upto, short: true } if (1..=packets).contains(&upto) => {
            let status = CTRL_TD + (n - 1) * TD_LEN;
            match run(m, c, CTRL_QH, status, 1, CONTROL_MS) {
                Ran::Ended { .. } => Ok(moved(m, CTRL_TD, 1, upto)),
                other => Err(failed(other)),
            }
        }
        other => Err(failed(other)),
    }
}

fn failed(r: Ran) -> CtrlError {
    match r {
        Ran::Stalled => CtrlError::Stalled,
        Ran::Error(bits) => CtrlError::Bus(bits),
        _ => CtrlError::TimedOut,
    }
}
