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

//! Bringing the controllers up without going deaf.
//!
//! The kernel names driver.i2c_pci0 the moment it spawns this capsule, so
//! the HID driver finds it and starts calling before any controller is up.
//! The shared bring-up sleeps between its attempts, and every call that
//! arrived meanwhile waited out its whole timeout and left an
//! `ipc.call unanswered` line on the console. Here the waits between
//! attempts serve the inbox instead: each call is answered at once with
//! E_BUSY, so the caller knows to come back later. The schedule is the
//! shared one (`nonos_libc::next`), so the attempts and the give-up are
//! unchanged.

use nonos_libc::bringup::{next, Next};
use nonos_libc::mk_debug;

use super::busy::answer_busy_for;

/// `attempt` until it succeeds or the shared schedule gives up, answering
/// every call between two attempts with E_BUSY.
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
            Next::Retry(ms) => answer_busy_for(ms),
            Next::GiveUp => {
                say_gave_up(driver, err);
                return Err(err);
            }
        }
    }
}

fn say_gave_up(driver: &[u8], reason: &str) {
    let mut line = alloc::vec::Vec::with_capacity(160);
    line.extend_from_slice(driver);
    line.extend_from_slice(b": device present, bring-up failed; not started (");
    line.extend_from_slice(reason.as_bytes());
    line.extend_from_slice(b")\n");
    let _ = mk_debug(line.as_ptr(), line.len());
}
