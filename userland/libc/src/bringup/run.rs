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

//! The bring-up loop every driver shares.

use super::deadline::retry_after;
use super::recv_turn::{recv_turn, RecvTurn, RECV_PARK_MS};
use super::say::say_gave_up;
use super::schedule::delay_after;
use crate::mk_idle_ms;

/// Try `attempt` until it succeeds or BRINGUP_BUDGET_MS have passed since
/// the first try began (deadline.rs), sleeping the schedule's delays between.
///
/// Each failed attempt must already have released whatever it claimed; the
/// next one claims afresh. The sleeps are real sleeps (`mk_idle_ms`), so a
/// device that never comes up costs a few wakeups, not a core. One line is
/// logged when the driver gives up, naming the driver and the last reason, and
/// nothing per attempt: the broker already logs each claim and release.
pub fn bring_up<T>(
    driver: &[u8],
    mut attempt: impl FnMut() -> Result<T, &'static str>,
) -> Result<T, &'static str> {
    let start = uptime();
    let mut done = 0u32;
    loop {
        let err = match attempt() {
            Ok(v) => return Ok(v),
            Err(e) => e,
        };
        done = done.saturating_add(1);
        match retry_after(uptime().saturating_sub(start), delay_after(done)) {
            Some(ms) => {
                let _ = mk_idle_ms(ms);
            }
            None => {
                say_gave_up(driver, err);
                return Err(err);
            }
        }
    }
}

/// Milliseconds since boot; a clock that will not say reads as 0.
fn uptime() -> u64 {
    u64::try_from(crate::mk_uptime_ms()).unwrap_or(0)
}

/// Whether a serve loop has a message to serve after its receive returned
/// `rc`. A receive that failed without waiting sleeps here first, so a loop
/// whose inbox is gone holds no core; one that waited goes round at once.
pub fn recv_ready(rc: i64) -> bool {
    match recv_turn(rc) {
        RecvTurn::Serve => true,
        RecvTurn::Again => false,
        RecvTurn::Park => {
            let _ = mk_idle_ms(RECV_PARK_MS);
            false
        }
    }
}
