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

//! Watching a started transaction to its end, within a bound. SMBus 2.0
//! (as recalled, unverified) lets a slave stretch the clock for 25 ms in all and calls a bus stuck
//! low for 35 ms timed out; a 32-byte block at the slowest 10 kHz clock is
//! about 35 ms more. 200 ms covers all of that with room, and a host that
//! takes longer is stopped with KILL (HST_CNT bit 1), which must then be
//! cleared before the host takes another transaction.

use super::status::{seen, Seen};
use crate::errno::E_TIMEDOUT;
use crate::ports::Ports;
use crate::regs::{CNT_KILL, HSTCNT, HSTSTS};

const BOUND_MS: u64 = 200;
const KILL_HELD_MS: u64 = 2;
const POLL_MS: u64 = 1;

/// The transaction's end as an answer: `Ok` on INTR, its error otherwise,
/// `E_TIMEDOUT` once killed.
pub fn watch<P: Ports>(p: &mut P) -> Result<(), i32> {
    p.arm(BOUND_MS);
    loop {
        match seen(p.r8(HSTSTS)?) {
            Seen::Finished => return Ok(()),
            Seen::Fault(e) => return Err(e),
            Seen::Running if p.expired() => break,
            Seen::Running => p.pause(POLL_MS),
        }
    }
    p.w8(HSTCNT, CNT_KILL)?;
    p.pause(KILL_HELD_MS);
    p.w8(HSTCNT, 0)?;
    Err(E_TIMEDOUT)
}
