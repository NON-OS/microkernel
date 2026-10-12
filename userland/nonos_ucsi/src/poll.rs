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

//! Waiting on the PPM: the CCI read after asking firmware to refresh the
//! mailbox, a pause between reads, and a fixed number of reads at most.

use super::exchange::Fault;
use super::layout::Cci;
use super::ppm::Ppm;

/// Polls before a command is called stuck: at one interval each, the time
/// a PPM may take before the session resets it.
pub const POLLS: u32 = 100;

pub fn note(pending: &mut u128, c: &Cci) {
    if c.connector != 0 {
        *pending |= 1u128 << c.connector;
    }
}

pub(super) fn poll(
    ppm: &mut impl Ppm,
    pending: &mut u128,
    done: fn(&Cci) -> bool,
) -> Result<Cci, Fault> {
    for _ in 0..POLLS {
        if !ppm.signal_read() {
            return Err(Fault::Signal);
        }
        let c = Cci::decode(ppm.read_cci().ok_or(Fault::Signal)?);
        note(pending, &c);
        if done(&c) {
            return Ok(c);
        }
        ppm.pause();
    }
    Err(Fault::Stuck)
}

pub(super) fn send(ppm: &mut impl Ppm, control: u64) -> Result<(), Fault> {
    (ppm.write_control(control) && ppm.signal_write()).then_some(()).ok_or(Fault::Signal)
}
