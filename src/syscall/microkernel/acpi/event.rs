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

//! `MkAcpiEvent(kind, value) -> 0`: the ACPI capsule reports a control-method
//! button, the lid, its hottest thermal zone, or a battery running low or
//! critical. A power button press goes to the desktop as the Power key. A
//! critical zone or battery asks the desktop to shut down, then, if it
//! stays critical, powers off through the same path as the admin shutdown.
//! EAGAIN when the input ring would not take the key.

use super::event_act::{act, pressed, say};
use event_owner::{lock, taken};

#[path = "event_owner.rs"]
mod event_owner;
use crate::hardware::broker::acpi::battery_watch::battery_critical;
use crate::hardware::broker::acpi::lid_state::{report as report_lid, Report};
use crate::hardware::broker::acpi::platform_event::{decode, Event};
use crate::hardware::broker::acpi::power_key::{brightness as brightness_key, lid as lid_key};
use crate::hardware::broker::acpi::thermal_watch::thermal;
use crate::syscall::microkernel::errnos::{ERRNO_AGAIN, ERRNO_INVAL};

pub fn sys_acpi_event(kind: u64, value: u64) -> i64 {
    let Some(event) = decode(kind, value) else { return ERRNO_INVAL };
    let now = crate::sys::clock::since_boot_ms();
    match event {
        Event::PowerButton => pressed(),
        Event::SleepButton => {
            say(b"[ACPI] sleep button pressed; NONOS has no sleep state yet");
            0
        }
        Event::Lid(open) => match report_lid(open, lid_key) {
            Report::Unchanged => 0,
            Report::Told => {
                say(if open { b"[ACPI] lid open" } else { b"[ACPI] lid closed" });
                0
            }
            Report::Busy => ERRNO_AGAIN,
        },
        Event::Thermal(w) => act(thermal(w, now), b"[ACPI] thermal zone at _CRT"),
        Event::BatteryLow => {
            say(b"[ACPI] battery low");
            0
        }
        Event::BatteryCritical(c) => act(battery_critical(c, now), b"[ACPI] battery critical"),
        Event::ButtonTaken => taken(),
        Event::GlobalLock(held) => lock(held),
        Event::Brightness(up) => match brightness_key(up) {
            true => 0,
            false => ERRNO_AGAIN,
        },
    }
}
