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

//! The outputs whose backlight ACPI controls: display output devices that
//! define both _BCL and _BCM (ACPI 6.5, B.6.2 and B.6.3), usually the
//! built-in panel under the graphics adapter's device.

use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::platform::scan;
use alloc::vec::Vec;

pub fn backlights(aml: &Aml) -> Vec<Id> {
    scan(aml, &|aml, id| {
        aml.ns.get(id).is_some_and(|e| e.kind == Kind::Device)
            && aml.ns.child(id, *b"_BCL").is_some()
            && aml.ns.child(id, *b"_BCM").is_some()
    })
}
