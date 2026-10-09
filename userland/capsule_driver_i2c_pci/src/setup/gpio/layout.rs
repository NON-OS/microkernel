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
use nonos_pinctrl::{
    amd_alive, amd_pin_reg, intel_padcfg0, locate, Line, AMD_PIN_STS, INTEL_PADBAR, INTEL_REVID,
    INTEL_RXSTATE,
};

use super::community::{claim, map};
use super::say::say;
use crate::constants::{GPIO_HID_BAR, HID_INFO_ACTIVE_HIGH};
use crate::discover::AcpiTouchpad;
use crate::driver::Doorbell;

/// The touchpad's pin on an Intel PCH from Sunrise Point on, or on AMD: the
/// controller record names its `_HID`, nonos_pinctrl turns the firmware pin
/// into a community window and a register, and that window alone is mapped.
/// Every refusal says on the log where it stopped; None means polling.
pub(super) fn doorbell(tp: &AcpiTouchpad) -> Option<Doorbell> {
    let uid = u16::from(tp.gpio_community.checked_sub(1)?);
    let Some((rec, epoch)) = claim(uid) else {
        return say(tp, "no GPIO controller record to claim for the touchpad's pin");
    };
    let hid = rec.bars[GPIO_HID_BAR].base.to_le_bytes();
    let found = match locate(&hid, u32::from(tp.gpio_pin)) {
        Ok(Line::Intel { bar, pad }) => map(&rec, epoch, bar).map(|(regs, len)| {
            let at = intel_padcfg0(regs.read32(INTEL_REVID), regs.read32(INTEL_PADBAR), pad, len);
            (regs, at.map_err(|e| e.why()), INTEL_RXSTATE)
        }),
        Ok(Line::Amd { pin }) => map(&rec, epoch, 0).map(|(regs, len)| {
            let at = amd_pin_reg(pin, len).and_then(|o| amd_alive(regs.read32(o)).map(|_| o));
            (regs, at.map_err(|e| e.why()), AMD_PIN_STS)
        }),
        Err(e) => {
            let _ = mk_device_release(rec.device_id);
            return say(tp, e.why());
        }
    };
    // map() released the claim when it failed; every other refusal does here.
    let Some((regs, at, level_bit)) = found else {
        return say(tp, "the community window would not map");
    };
    let cfg_offset = match at {
        Ok(o) => o,
        Err(why) => {
            let _ = mk_device_release(rec.device_id);
            return say(tp, why);
        }
    };
    let _ = say(tp, &alloc::format!("level read at {:#x} bit {:#x}", cfg_offset, level_bit));
    let active_high = tp.info & HID_INFO_ACTIVE_HIGH != 0;
    Some(Doorbell { regs, cfg_offset, level_bit, active_high })
}
