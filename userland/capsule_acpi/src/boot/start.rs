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

//! The capsule's start: tables and namespace, then the first publish pass
//! (`first`) that init waits on, then everything that may take longer
//! (`late`): events, batteries, cooling, processors, Type-C.

use super::pass::{FirstPass, Step};
use super::say::say_ec;
use super::state::Acpi;
use super::{assemble, bringup, first, load, report, tables};
use crate::host::CapsuleHost;

pub fn boot() -> Option<Acpi> {
    let smbios = crate::smbios::read();
    let t = tables::fetch()?;
    let mut host = CapsuleHost { facs: tables::map_facs(&t.fadt), ..CapsuleHost::default() };
    let mut aml = load::load(&t, &mut host);
    // Found, not served: its region must be reachable while _REG and _INI run.
    let ec = crate::ec::find(&mut aml, &mut host);
    host.ec = ec.as_ref().map(|e| (e.ports, e.glk));
    say_ec(ec.as_ref());
    let grants = bringup::bring_up(&mut aml, &mut host);
    let mut pass = FirstPass::default();
    pass.finish(Step::Namespace);
    pass.finish(Step::Osc);
    let (devices, routes) = report::read(&mut aml, &mut host);
    crate::sleep::read_s5(&mut aml, &mut host);
    let serial = crate::serial::find(&devices, &mut aml, &mut host);
    let legacy = crate::legacy::find(&t.fadt, &devices, &mut aml, &mut host);
    let parts = assemble::Parts { aml, host, ec, devices, routes, serial, legacy, smbios };
    let mut acpi = assemble::assemble(parts);
    first::first(&mut acpi, &mut pass);
    super::late::late(&mut acpi, &t.fadt, &grants);
    Some(acpi)
}
