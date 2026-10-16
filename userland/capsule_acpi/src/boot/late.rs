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

//! What runs once everything else is up: the ranges firmware reserves and
//! the root bridges' proximity domains, the functions _PS0 woke, the
//! platform controllers and port facts published to the broker, and the
//! Type-C session, whose first walk may take a while on a slow PPM.

use super::state::Acpi;

pub fn late(acpi: &mut Acpi) {
    crate::reserved::publish(acpi);
    crate::numa::publish(acpi);
    crate::dpower::reveal_hidden(acpi);
    crate::hotplug::learn(acpi);
    let Acpi { devices, aml, host, .. } = acpi;
    crate::platform::publish(devices, aml, host);
    crate::ports::publish(acpi);
    acpi.typec = crate::typec::find(&acpi.devices, &mut acpi.aml, &mut acpi.host);
}
