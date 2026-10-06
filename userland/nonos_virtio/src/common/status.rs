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

//! The device status byte: reset, then ACKNOWLEDGE and DRIVER, features,
//! FEATURES_OK, queues, DRIVER_OK; FAILED on any step that does not hold.

use super::access::CommonCfg;
use super::regs::DEVICE_STATUS;
use crate::error::VirtioError;

pub const STATUS_ACKNOWLEDGE: u8 = 1;
pub const STATUS_DRIVER: u8 = 2;
pub const STATUS_DRIVER_OK: u8 = 4;
pub const STATUS_FEATURES_OK: u8 = 8;
pub const STATUS_FAILED: u8 = 0x80;

/// How many times the status is read back after a reset. A modern device
/// may finish its reset after the write returns, and the specification has
/// the driver wait for zero before going on; QEMU is done at once.
pub const RESET_POLLS: u32 = 1000;

pub fn reset(c: &impl CommonCfg) -> Result<(), VirtioError> {
    c.w8(DEVICE_STATUS, 0);
    for _ in 0..RESET_POLLS {
        if c.r8(DEVICE_STATUS) == 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(VirtioError::ResetTimeout)
}

/// Reset, then tell the device it was seen and that a driver is here.
pub fn start(c: &impl CommonCfg) -> Result<(), VirtioError> {
    reset(c)?;
    add(c, STATUS_ACKNOWLEDGE);
    add(c, STATUS_DRIVER);
    Ok(())
}

/// Give up on the device; it may not be driven until reset.
pub fn fail(c: &impl CommonCfg) {
    add(c, STATUS_FAILED);
}

pub fn driver_ok(c: &impl CommonCfg) {
    add(c, STATUS_DRIVER_OK);
}

fn add(c: &impl CommonCfg, bits: u8) {
    let s = c.r8(DEVICE_STATUS);
    c.w8(DEVICE_STATUS, s | bits);
}
