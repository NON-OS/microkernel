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

use super::policy::{decide, next, recv_turn, Next, RecvTurn, RECV_PARK_MS};
use crate::{mk_debug, mk_idle_ms};

/// Try `attempt` until it succeeds or the schedule in `policy` runs out.
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
    let mut done = 0u32;
    loop {
        let err = match attempt() {
            Ok(v) => return Ok(v),
            Err(e) => e,
        };
        done = done.saturating_add(1);
        match next(done) {
            Next::Retry(ms) => {
                let _ = mk_idle_ms(ms);
            }
            Next::GiveUp => {
                say_gave_up(driver, err);
                return Err(err);
            }
        }
    }
}

/// A driver's whole start: leave at once, saying so, when discovery found
/// nothing (`found` is `None`); otherwise bring the device up within the
/// bounded schedule. `Err` carries the code for `mk_exit`, `EXIT_ABSENT` or
/// `EXIT_GAVE_UP`, and by then nothing the driver claimed is still held.
pub fn start_driver<F, T>(
    driver: &[u8],
    found: Option<F>,
    mut attempt: impl FnMut(&F) -> Result<T, &'static str>,
) -> Result<T, i32> {
    decide(found, || say_absent(driver), |device| bring_up(driver, || attempt(&device)))
}

/// The one line a driver logs when the machine has none of its devices.
pub fn say_absent(driver: &[u8]) {
    say(&[driver, b": no controller present, not started"]);
}

fn say_gave_up(driver: &[u8], reason: &str) {
    say(&[driver, b": device present, bring-up failed; not started (", reason.as_bytes(), b")"]);
}

/// One console line from `parts`, cut to fit and always ending in a newline,
/// so the next line on the serial console starts on its own.
fn say(parts: &[&[u8]]) {
    let mut line = [0u8; 160];
    let room = line.len() - 1;
    let mut n = 0usize;
    for part in parts {
        for &b in *part {
            if n == room {
                break;
            }
            line[n] = b;
            n += 1;
        }
    }
    line[n] = b'\n';
    let _ = mk_debug(line.as_ptr(), n + 1);
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
