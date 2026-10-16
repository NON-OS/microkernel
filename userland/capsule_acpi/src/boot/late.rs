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

//! What runs once the first publish pass is told: the events (the EC's
//! with them), the batteries and AC, the buttons and lid, the cooling,
//! the processors, backlight, hotkeys and hardware errors; any hidden
//! function the pass had no time for; the hot-plug ports _OSC gave NONOS;
//! port facts; and the Type-C session, whose first walk may take a while.

use super::state::Acpi;
use crate::hotplug::native::osc_grant::OscGrant;

pub fn late(acpi: &mut Acpi, fadt: &[u8], grants: &[OscGrant]) {
    let gpe = acpi.ec.as_ref().and_then(|e| e.gpe);
    let (events, sci) = super::sci::start(fadt, &mut acpi.aml, &mut acpi.host, gpe);
    (acpi.events, acpi.sci) = (events, sci);
    acpi.power = crate::power::find(&acpi.devices);
    crate::power::all(&mut acpi.power, &mut acpi.aml, &mut acpi.host);
    acpi.buttons = crate::buttons::find(&acpi.devices);
    crate::buttons::read_lid(&mut acpi.buttons, &mut acpi.aml, &mut acpi.host);
    // The pool already counts what the first pass powered.
    let pool = core::mem::take(&mut acpi.cooling.pool);
    acpi.cooling = crate::cooling::find(&mut acpi.aml, &mut acpi.host);
    acpi.cooling.pool = pool;
    acpi.backlight = crate::backlight::find(&mut acpi.aml, &mut acpi.host);
    acpi.cpus = crate::cpu::find(&mut acpi.aml, &mut acpi.host);
    acpi.hotkeys = crate::hotkeys::find(&acpi.devices, &mut acpi.aml, &mut acpi.host);
    acpi.apei = crate::apei::start(&acpi.devices);
    super::say::say_cooling(&acpi.cooling);
    crate::dpower::reveal_hidden(acpi, u64::MAX);
    crate::hotplug::native::find(acpi, grants);
    crate::ports::publish(acpi);
    acpi.typec = crate::typec::find(&acpi.devices, &mut acpi.aml, &mut acpi.host);
}
