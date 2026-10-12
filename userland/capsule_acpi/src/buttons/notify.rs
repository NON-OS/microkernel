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

//! A Notify on a button or lid (ACPI 6.5, 5.6.6): 0x80 is a press, or the
//! lid moving; 0x02 says the device woke the machine. A press the kernel
//! could not post yet is tried a few times before it is given up.

use super::lid::read_lid;
use super::state::Buttons;
use crate::host::CapsuleHost;
use crate::sys::acpi_event::{acpi_event, AGAIN, POWER_BUTTON, SLEEP_BUTTON};
use crate::sys::clock::sleep_ms;
use crate::sys::log::Line;
use nonos_aml::button::{DEVICE_WAKE, STATUS_CHANGE};
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;

const TRIES: u32 = 5;
const RETRY_MS: u64 = 20;

fn press(kind: u64, what: &[u8]) {
    Line::new(what).str(b" button pressed").end();
    for _ in 0..TRIES {
        match acpi_event(kind, 0) {
            Err(AGAIN) => sleep_ms(RETRY_MS),
            _ => return,
        }
    }
    Line::new(what).str(b" button: the desktop's input was busy; press dropped").end();
}

pub fn notified(b: &mut Buttons, aml: &mut Aml, host: &mut CapsuleHost, id: Id, value: u8) -> bool {
    let (power, sleep, lid) = (b.power.contains(&id), b.sleep.contains(&id), b.lids.contains(&id));
    match value {
        _ if !(power || sleep || lid) => return false,
        DEVICE_WAKE => Line::new(b"woken by a button or the lid").end(),
        STATUS_CHANGE if power => press(POWER_BUTTON, b"power"),
        STATUS_CHANGE if sleep => press(SLEEP_BUTTON, b"sleep"),
        STATUS_CHANGE => read_lid(b, aml, host),
        v => Line::new(b"button notify ").hex(u64::from(v)).end(),
    }
    true
}
