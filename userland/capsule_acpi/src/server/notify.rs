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

//! Each Notify the AML queued, to whoever serves its object: the batteries
//! and adapters, the buttons and lids, the thermal zones and fans, the
//! backlit outputs. Anything
//! else is logged.

use crate::boot::{path, Acpi};
use crate::sys::log::Line;

pub fn route(acpi: &mut Acpi) {
    while let Some((id, value)) = acpi.aml.rt.notifications.pop_front() {
        let Acpi { aml, host, power, buttons, cooling, backlight, .. } = acpi;
        let served = crate::power::notified(power, aml, host, id, value)
            || crate::buttons::notified(buttons, aml, host, id, value)
            || crate::cooling::notified(cooling, aml, host, id, value)
            || crate::backlight::notified(backlight, aml, host, id, value);
        if !served {
            Line::new(b"notify ").str(path(aml, id).as_bytes()).str(b" ").hex(value as u64).end();
        }
    }
}
