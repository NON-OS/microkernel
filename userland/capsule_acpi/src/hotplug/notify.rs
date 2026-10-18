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

//! The Notify values hot-plug serves (`nonos_aml::hotplug::check`), on a
//! device that can come and go: one with an `_ADR`, a dock, or one firmware
//! can eject. Anything else is left to the other servers.

use super::look::look;
use super::pending::request;
use crate::boot::Acpi;
use nonos_aml::hotplug::{check, ejectable, is_dock, Check};
use nonos_aml::namespace::{Id, Kind};

pub fn notified(acpi: &mut Acpi, id: Id, value: u8) -> bool {
    let Some(c) = check(value) else { return false };
    let aml = &acpi.aml;
    let device = aml.ns.get(id).is_some_and(|e| e.kind == Kind::Device);
    let comes_and_goes =
        aml.ns.child(id, *b"_ADR").is_some() || is_dock(aml, id) || ejectable(aml, id);
    if !device || !comes_and_goes {
        return false;
    }
    match c {
        Check::Eject => request(acpi, id),
        c => look(acpi, id, c.deep()),
    }
    true
}
