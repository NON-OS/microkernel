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
//! button, the lid, or its hottest thermal zone. A power button press goes
//! to the desktop as the Power key; a critical zone asks the desktop to shut
//! down, then, if it stays critical, powers off through the same path as
//! the admin shutdown. EAGAIN when the input ring would not take the key.

use crate::arch::power::PowerOff;
use crate::hardware::broker::acpi::lid_state::record as record_lid;
use crate::hardware::broker::acpi::platform_event::{decode, Event};
use crate::hardware::broker::acpi::power_key::press;
use crate::hardware::broker::acpi::thermal_watch::{thermal, Action};
use crate::security::zerostate::terminate;
use crate::syscall::microkernel::errnos::{ERRNO_AGAIN, ERRNO_INVAL};

fn say(line: &[u8]) {
    crate::sys::serial::println(line);
}

fn pressed() -> i64 {
    if press() {
        0
    } else {
        ERRNO_AGAIN
    }
}

pub fn sys_acpi_event(kind: u64, value: u64) -> i64 {
    let Some(event) = decode(kind, value) else { return ERRNO_INVAL };
    match event {
        Event::PowerButton => pressed(),
        Event::SleepButton => {
            say(b"[ACPI] sleep button pressed; NONOS has no sleep state yet");
            0
        }
        Event::Lid(open) => {
            if record_lid(open) {
                say(if open { b"[ACPI] lid open" } else { b"[ACPI] lid closed" });
            }
            0
        }
        Event::Thermal(w) => match thermal(w, crate::sys::clock::since_boot_ms()) {
            Action::Nothing => 0,
            Action::AskShutdown => {
                say(b"[ACPI] thermal zone critical: asking the desktop to shut down");
                pressed()
            }
            Action::PowerOff => {
                say(b"[ACPI] thermal zone still critical: powering off");
                terminate(PowerOff::Shutdown)
            }
        },
    }
}
