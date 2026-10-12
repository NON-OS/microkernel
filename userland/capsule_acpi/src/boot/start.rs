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

use super::state::Acpi;
use super::{bringup, load, report, sci, tables};
use crate::host::CapsuleHost;

pub fn boot() -> Option<Acpi> {
    let t = tables::fetch()?;
    let mut host = CapsuleHost { facs: tables::map_facs(&t.fadt), ..CapsuleHost::default() };
    let mut aml = load::load(&t, &mut host);
    bringup::bring_up(&mut aml, &mut host);
    let (devices, routes) = report::read(&mut aml, &mut host);
    let (events, sci) = sci::start(&t.fadt, &mut aml, &mut host);
    Some(Acpi { aml, host, events, sci, devices, routes, scis: 0, gpe_runs: 0 })
}
