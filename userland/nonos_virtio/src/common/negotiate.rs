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

//! The 64 feature bits, read and written as two selected 32-bit pages, and
//! the FEATURES_OK step that commits them.

use super::access::CommonCfg;
use super::regs::{
    DEVICE_FEATURE, DEVICE_FEATURE_SELECT, DEVICE_STATUS, DRIVER_FEATURE, DRIVER_FEATURE_SELECT,
};
use super::status::{fail, STATUS_FEATURES_OK};
use crate::error::VirtioError;
use crate::features::negotiate;

pub fn device_features(c: &impl CommonCfg) -> u64 {
    c.w32(DEVICE_FEATURE_SELECT, 0);
    let low = c.r32(DEVICE_FEATURE) as u64;
    c.w32(DEVICE_FEATURE_SELECT, 1);
    let high = c.r32(DEVICE_FEATURE) as u64;
    (high << 32) | low
}

pub fn write_driver_features(c: &impl CommonCfg, features: u64) {
    c.w32(DRIVER_FEATURE_SELECT, 0);
    c.w32(DRIVER_FEATURE, features as u32);
    c.w32(DRIVER_FEATURE_SELECT, 1);
    c.w32(DRIVER_FEATURE, (features >> 32) as u32);
}

/// Negotiate against what the device offers, write the result, set
/// FEATURES_OK and read it back: a device that clears it has refused the
/// set. Either refusal leaves the device marked FAILED. Returns the
/// features in force.
pub fn accept(c: &impl CommonCfg, driver_handles: u64) -> Result<u64, VirtioError> {
    let accepted = match negotiate(device_features(c), driver_handles) {
        Ok(f) => f,
        Err(e) => {
            fail(c);
            return Err(e);
        }
    };
    write_driver_features(c, accepted);
    let s = c.r8(DEVICE_STATUS);
    c.w8(DEVICE_STATUS, s | STATUS_FEATURES_OK);
    if c.r8(DEVICE_STATUS) & STATUS_FEATURES_OK == 0 {
        fail(c);
        return Err(VirtioError::FeaturesRejected);
    }
    Ok(accepted)
}
