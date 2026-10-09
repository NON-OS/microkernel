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

//! The return to transfer state after a failed command.

use super::super::super::env::{Clock, Deadline, Log, Mmio};
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::cmds::STOP_TRANSMISSION;
use super::super::r1::{state, STATE_DATA, STATE_RCV, STATE_TRAN};
use super::status::status;

/// Time to bring the card back to transfer state after a failure.
pub const RECOVER_MS: u64 = 1_000;
/// Busy wait for CMD12.
pub const STOP_MS: u64 = 500;

/// Bring the card back to transfer state after a failed command: while it
/// is still sending or receiving data, STOP_TRANSMISSION (as an abort, at
/// most three times); while programming, wait. True once it is in transfer
/// state.
pub fn recover<M: Mmio, C: Clock, L: Log>(h: &mut Host<M, C, L>, rca: u16) -> bool {
    let d = Deadline::after(&h.clock, RECOVER_MS);
    let mut stops = 0;
    let mut last = None;
    loop {
        if let Ok(s) = status(h, rca) {
            last = Some(s);
            match state(s) {
                STATE_TRAN => return true,
                STATE_DATA | STATE_RCV if stops < 3 => {
                    let mut c = Cmd::new(STOP_TRANSMISSION, 0, Resp::R1b);
                    c.abort = true;
                    c.wait_ms = STOP_MS;
                    let _ = h.send(&c);
                    stops += 1;
                    continue;
                }
                _ => {}
            }
        }
        if d.passed(&h.clock) {
            let mut l = Line::new();
            l.s(b"card did not return to transfer state");
            if let Some(s) = last {
                l.s(b" (status ").hex(s as u64).s(b")");
            }
            h.say(&l);
            return false;
        }
        h.pause(1);
    }
}
