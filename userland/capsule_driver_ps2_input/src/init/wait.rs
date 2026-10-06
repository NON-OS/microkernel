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

//! Waiting on the i8042 status register, bounded in time. The bounds were
//! spin counts once; a port read is a system call whose cost differs by an
//! order of magnitude between QEMU and an EC-emulated controller on a laptop,
//! so a count that waited long enough on one returned early on the other.
//! The limits follow Linux: i8042_wait_read/write give the controller half a
//! second, libps2 gives a device 200 ms to acknowledge a byte and longer to
//! answer a command.

use nonos_libc::{mk_pio_read, Deadline};

use crate::constants::{STATUS_INPUT_FULL, STATUS_OFFSET};

/// The controller takes a command or data byte (input buffer empty).
pub const CTL_TIMEOUT_MS: u64 = 500;
/// A device acknowledges a byte it was sent.
pub const ACK_TIMEOUT_MS: u64 = 200;
/// A device answers a command (an id, a configuration byte).
pub const REPLY_TIMEOUT_MS: u64 = 500;
/// A keyboard finishes its self test after a reset. Real keyboards take
/// 300 to 500 ms; Linux allows a slow one far longer.
pub const BAT_TIMEOUT_MS: u64 = 2_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WaitError {
    /// The broker refused the port read.
    Read,
    /// The status bit never moved within the bound.
    Timeout,
}

fn wait(grant_id: u64, timeout_ms: u64, done: impl Fn(u8) -> bool) -> Result<(), WaitError> {
    let deadline = Deadline::after_ms(timeout_ms);
    loop {
        let mut status = 0u32;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return Err(WaitError::Read);
        }
        if done(status as u8) {
            return Ok(());
        }
        if deadline.expired() {
            return Err(WaitError::Timeout);
        }
        core::hint::spin_loop();
    }
}

/// Until the controller's input buffer is empty and it takes a byte.
pub fn wait_input_clear(grant_id: u64, timeout_ms: u64) -> Result<(), WaitError> {
    wait(grant_id, timeout_ms, |s| s & STATUS_INPUT_FULL == 0)
}
