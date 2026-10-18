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
use super::reset::{reset, reset_until};
use crate::error::VirtioError;

pub const STATUS_ACKNOWLEDGE: u8 = 1;
pub const STATUS_DRIVER: u8 = 2;
pub const STATUS_DRIVER_OK: u8 = 4;
pub const STATUS_FEATURES_OK: u8 = 8;
pub const STATUS_FAILED: u8 = 0x80;

/// Reset, then tell the device it was seen and that a driver is here.
pub fn start(c: &impl CommonCfg) -> Result<(), VirtioError> {
    reset(c)?;
    announce(c);
    Ok(())
}

/// `start`, with the reset waited for until `expired` (see `reset_until`).
pub fn start_until(c: &impl CommonCfg, expired: impl FnMut() -> bool) -> Result<(), VirtioError> {
    reset_until(c, expired)?;
    announce(c);
    Ok(())
}

fn announce(c: &impl CommonCfg) {
    add(c, STATUS_ACKNOWLEDGE);
    add(c, STATUS_DRIVER);
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
