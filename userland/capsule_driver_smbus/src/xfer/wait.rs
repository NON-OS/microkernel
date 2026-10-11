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

//! Waiting for the host to finish, bounded, and stopping it when it does not.

use crate::errno::E_TIMEDOUT;
use crate::ports::Ports;
use crate::regs::{CNT_KILL, HSTCNT, HSTSTS, STS_ERROR, STS_HOST_BUSY, STS_INTR};

/// Linux's adapter timeout, HZ / 5.
const TIMEOUT_MS: u64 = 200;
/// How long the kill is held before the control register is cleared.
const KILL_MS: u64 = 2;
const POLL_MS: u64 = 1;

/// The status once the host is idle with a result, or `E_TIMEDOUT` after
/// the transaction was killed (Linux's i801_wait_intr and its timeout path).
pub fn wait_done<P: Ports>(p: &mut P) -> Result<u8, i32> {
    p.arm(TIMEOUT_MS);
    loop {
        let st = p.r8(HSTSTS)?;
        if st & STS_HOST_BUSY == 0 && st & (STS_ERROR | STS_INTR) != 0 {
            return Ok(st);
        }
        if p.expired() {
            p.w8(HSTCNT, CNT_KILL)?;
            p.pause(KILL_MS);
            p.w8(HSTCNT, 0)?;
            return Err(E_TIMEDOUT);
        }
        p.pause(POLL_MS);
    }
}
