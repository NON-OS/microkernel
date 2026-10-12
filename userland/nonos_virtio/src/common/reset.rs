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

//! Resetting the device: write zero to the status and wait for it to read
//! back zero. A modern device may finish after the write returns, and the
//! specification has the driver wait.

use super::access::CommonCfg;
use super::regs::DEVICE_STATUS;
use crate::error::VirtioError;

/// How many times `reset` reads the status back. A count runs at the
/// processor's speed, so a driver with a clock waits with `reset_until`.
pub const RESET_POLLS: u32 = 1000;

/// Reset, giving up once `expired` says so. `expired` is asked after each
/// read that was not yet zero, so it may sleep; the crate has no clock, so
/// the driver's own (a deadline) bounds the wait.
pub fn reset_until(
    c: &impl CommonCfg,
    mut expired: impl FnMut() -> bool,
) -> Result<(), VirtioError> {
    c.w8(DEVICE_STATUS, 0);
    loop {
        if c.r8(DEVICE_STATUS) == 0 {
            return Ok(());
        }
        if expired() {
            return Err(VirtioError::ResetTimeout);
        }
    }
}

/// Reset, reading the status back at most RESET_POLLS times.
pub fn reset(c: &impl CommonCfg) -> Result<(), VirtioError> {
    let mut polls = 0u32;
    reset_until(c, || {
        polls += 1;
        core::hint::spin_loop();
        polls >= RESET_POLLS
    })
}
