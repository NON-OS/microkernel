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

//! Low and critical batteries, told to the kernel. Low is the charge at or
//! under the battery's warning capacity while discharging, said once per
//! time it happens. Critical is _BST's critical bit, or the charge at or
//! under the low capacity, while discharging (ACPI 6.5, 10.2.2.11): said
//! at every reading while it lasts, so the kernel's watch sees the time
//! pass, and once more when it ends. Becoming critical runs the wiped
//! power-off at once (F-88); the kernel's watch is what is left if refused.
//! "Discharging" includes a direction firmware made unknown by claiming
//! both: a wrong "not discharging" would cost a critical battery its
//! power-off.

use super::state::{Battery, Power};
use crate::power::off::rule::battery_critical;
use crate::sys::acpi_event::{acpi_event, BATTERY_CRITICAL, BATTERY_LOW};
use crate::sys::log::Line;

fn under(b: &Battery, level: fn(&nonos_aml::power::BatteryInfo) -> Option<u32>) -> bool {
    let (Some(i), Some(s)) = (&b.info, &b.status) else { return false };
    s.may_discharge() && matches!((s.remaining, level(i)), (Some(r), Some(l)) if r <= l)
}

fn critical(b: &Battery) -> bool {
    let low = b.info.as_ref().and_then(|i| i.low);
    b.status.is_some_and(|s| battery_critical(s.may_discharge(), s.critical, s.remaining, low))
}

pub fn check(p: &mut Power) {
    let present = || p.batteries.iter().filter(|b| b.present);
    let low = present().any(|b| under(b, |i| i.warning));
    let critical = present().any(critical);
    if low && !p.low_told && acpi_event(BATTERY_LOW, 0).is_ok() {
        Line::new(b"battery low").end();
        p.low_told = true;
    }
    p.low_told &= low;
    if critical || p.critical {
        if critical && !p.critical {
            Line::new(b"battery critical").end();
            // Refused, the kernel's grace power-off below still runs.
            crate::power::off::power_off(crate::power::off::Why::Battery);
        }
        if acpi_event(BATTERY_CRITICAL, u64::from(critical)).is_ok() {
            p.critical = critical;
        }
    }
}
