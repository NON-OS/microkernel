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

use nonos_libc::mk_device_release;

use super::community::{claim, map};
use crate::constants::{
    bxt_pad_offset, is_bxt_family, GPIO_PADBAR, GPIO_RXSTATE, HID_INFO_ACTIVE_HIGH, HID_INFO_GPIO,
};
use crate::discover::AcpiTouchpad;
use crate::driver::Doorbell;

/// Map the touchpad's GPIO community and locate its pad, when the platform
/// is one whose layout is known. None means the HID driver polls.
pub(super) fn doorbell(pci_device: u16, tp: &AcpiTouchpad) -> Option<Doorbell> {
    if tp.info & HID_INFO_GPIO == 0 || !is_bxt_family(pci_device) {
        return None;
    }
    let uid = u16::from(tp.gpio_community.checked_sub(1)?);
    let (rec, epoch) = claim(uid)?;
    let (regs, size) = map(&rec, epoch, 0)?;
    let Some(cfg_offset) = bxt_pad_offset(regs.read32(GPIO_PADBAR), tp.gpio_pin, size) else {
        let _ = mk_device_release(rec.device_id);
        return None;
    };
    let active_high = tp.info & HID_INFO_ACTIVE_HIGH != 0;
    Some(Doorbell { regs, cfg_offset, level_bit: GPIO_RXSTATE, active_high })
}
