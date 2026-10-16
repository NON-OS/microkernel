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

//! Stopping a device-tree DesignWare I2C controller when its capsule lets it
//! go: IC_ENABLE cleared, then IC_ENABLE_STATUS read until the controller
//! reports itself off, within a bound. A controller in the middle of a byte
//! finishes it first. Offsets and bits are the DW_apb_i2c databook's, as
//! recalled, unverified: IC_ENABLE 6Ch bit 0, IC_ENABLE_STATUS 9Ch bit 0.

pub(crate) const IC_ENABLE: usize = 0x6C;
pub(crate) const IC_ENABLE_STATUS: usize = 0x9C;
const ENABLED: u32 = 1;
/// Looks at IC_ENABLE_STATUS, and the pause between them: 10 ms in all,
/// many bytes at the slowest standard-mode rate.
const LOOKS: u32 = 100;
const PAUSE_US: u32 = 100;

/// The controller's registers, and a pause.
pub(crate) trait DwRegs {
    fn read32(&self, offset: usize) -> u32;
    fn write32(&self, offset: usize, value: u32);
    fn pause_us(&self, us: u32);
}

/// True once the controller reports itself off; false when it is still on
/// after the bound, which the caller logs before it takes the window back.
pub(crate) fn stop(r: &impl DwRegs) -> bool {
    r.write32(IC_ENABLE, 0);
    for _ in 0..LOOKS {
        if r.read32(IC_ENABLE_STATUS) & ENABLED == 0 {
            return true;
        }
        r.pause_us(PAUSE_US);
    }
    false
}
