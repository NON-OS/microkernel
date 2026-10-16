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

//! Docking and a dock found gone. `_DCK(1)` docks; only a dock it reports
//! docked has what is below it looked at. A dock that is gone without an
//! eject took everything that names it in `_EJD` with it.

use super::leave::leave;
use crate::boot::{evaluator, path, Acpi};
use crate::sys::log::Line;
use nonos_aml::hotplug::{dependents, dock, targets};
use nonos_aml::namespace::Id;

/// Whether the dock is docked (already, or now by `_DCK(1)`).
pub fn docked(acpi: &mut Acpi, id: Id) -> bool {
    if acpi.hotplug.docked.contains(&id) {
        return true;
    }
    let r = dock(&mut acpi.aml, &mut evaluator(&mut acpi.host), id, true);
    let l = Line::new(b"hotplug ").str(path(&acpi.aml, id).as_bytes());
    let ok = matches!(r, Some(Ok(true)));
    if ok {
        acpi.hotplug.docked.push(id);
        l.str(b": docked").end();
    } else {
        l.str(b": _DCK(1) did not dock").end();
    }
    ok
}

/// Everything that names the dock in `_EJD`, and all below each.
pub fn with_dock(acpi: &mut Acpi, id: Id) -> alloc::vec::Vec<Id> {
    let deps = dependents(&mut acpi.aml, &mut evaluator(&mut acpi.host), id);
    deps.into_iter().flat_map(|d| targets(&acpi.aml, d, true)).collect()
}

pub fn undocked(acpi: &mut Acpi, id: Id) {
    if !acpi.hotplug.docked.contains(&id) {
        return;
    }
    acpi.hotplug.docked.retain(|d| *d != id);
    for d in with_dock(acpi, id) {
        leave(acpi, d);
    }
    Line::new(b"hotplug ").str(path(&acpi.aml, id).as_bytes()).str(b": undocked").end();
}
