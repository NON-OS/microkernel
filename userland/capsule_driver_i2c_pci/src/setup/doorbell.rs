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
//! The touchpad's interrupt line named for driver.gpio0: this driver's
//! device key, and the line's GPIO controller `_UID` and pin from the
//! firmware's GpioInt. The key is the broker device id of the controller
//! this driver claimed, with bit 63 set (the lead's ruling): a capsule is
//! never shown a PCI function's bus, device and function, so the PCI form
//! of the key is not one it can build, and the device id names the claim
//! gpio0's holder check tests (F 62).

use crate::constants::HID_INFO_GPIO;
use crate::discover::AcpiTouchpad;
use crate::driver::Doorbell;

/// A key naming a broker device id rather than a PCI function.
pub const KEY_DEVICE_ID: u64 = 1 << 63;

pub fn doorbell(device_id: u64, tp: &AcpiTouchpad) -> Option<Doorbell> {
    if tp.info & HID_INFO_GPIO == 0 || device_id & KEY_DEVICE_ID != 0 {
        return None;
    }
    let uid = u16::from(tp.gpio_community.checked_sub(1)?);
    Some(Doorbell { key: KEY_DEVICE_ID | device_id, uid, pin: tp.gpio_pin })
}
