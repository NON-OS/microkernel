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

//! The capsule's start: tables, namespace, devices, events.

use super::say::{say_cooling, say_ec};
use super::state::Acpi;
use super::{bringup, load, report, sci, tables};
use crate::host::CapsuleHost;

pub fn boot() -> Option<Acpi> {
    let smbios = crate::smbios::read();
    let t = tables::fetch()?;
    let mut host = CapsuleHost { facs: tables::map_facs(&t.fadt), ..CapsuleHost::default() };
    let mut aml = load::load(&t, &mut host);
    let ec = crate::ec::find(&mut aml, &mut host);
    host.ec = ec.as_ref().map(|e| (e.ports, e.glk));
    say_ec(ec.as_ref());
    bringup::bring_up(&mut aml, &mut host);
    let (devices, routes) = report::read(&mut aml, &mut host);
    crate::sleep::read_s5(&mut aml, &mut host);
    let (events, sci) = sci::start(&t.fadt, &mut aml, &mut host, ec.as_ref().and_then(|e| e.gpe));
    let mut power = crate::power::find(&devices);
    crate::power::all(&mut power, &mut aml, &mut host);
    let mut buttons = crate::buttons::find(&devices);
    crate::buttons::read_lid(&mut buttons, &mut aml, &mut host);
    let cooling = crate::cooling::find(&mut aml, &mut host);
    say_cooling(&cooling);
    let backlight = crate::backlight::find(&mut aml, &mut host);
    let cpus = crate::cpu::find(&mut aml, &mut host);
    let hotkeys = crate::hotkeys::find(&devices, &mut aml, &mut host);
    let serial = crate::serial::find(&devices, &mut aml, &mut host);
    let (scis, gpe_runs) = (0, 0);
    Some(Acpi {
        aml,
        host,
        events,
        sci,
        ec,
        power,
        buttons,
        cooling,
        cpus,
        dpower: Default::default(),
        backlight,
        hotkeys,
        serial,
        smbios,
        devices,
        routes,
        scis,
        gpe_runs,
    })
}
