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

//! What MkAcpiEvent does with a press or a critical report: the Power key
//! posted, and, when a critical condition outlasts its grace, the admin
//! shutdown path.

use crate::arch::power::PowerOff;
use crate::hardware::broker::acpi::critical_watch::Action;
use crate::hardware::broker::acpi::power_key::press;
use crate::security::zerostate::terminate;
use crate::syscall::microkernel::errnos::ERRNO_AGAIN;

pub(super) fn say(line: &[u8]) {
    crate::sys::serial::println(line);
}

pub(super) fn pressed() -> i64 {
    if press() {
        0
    } else {
        ERRNO_AGAIN
    }
}

pub(super) fn act(action: Action, what: &[u8]) -> i64 {
    match action {
        Action::Nothing => 0,
        Action::AskShutdown => {
            say(what);
            say(b"[ACPI] critical: asking the desktop to shut down");
            pressed()
        }
        Action::PowerOff => {
            say(what);
            say(b"[ACPI] still critical: powering off");
            terminate(PowerOff::Shutdown)
        }
    }
}
