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

//! How a fan is run (ACPI 6.5, 11.3): through _FSL when it has one and
//! its levels for on and off are known from _FIF and _FPS; else by putting
//! it in D0 or D3, as an ACPI 1.0 fan is.

use super::state::Drive;
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use alloc::vec::Vec;
use nonos_aml::fan::{levels, parse_fif, parse_fps};
use nonos_aml::load::Aml;
use nonos_aml::namespace::{Id, Seg};
use nonos_aml::object::Object;

fn get(aml: &mut Aml, host: &mut CapsuleHost, dev: Id, seg: Seg) -> Option<Object> {
    let id = aml.ns.child(dev, seg)?;
    evaluator(host)(aml, id, Vec::new()).ok()
}

pub(super) fn drive_of(aml: &mut Aml, host: &mut CapsuleHost, dev: Id) -> Drive {
    if aml.ns.child(dev, *b"_FSL").is_none() {
        return Drive::Power;
    }
    let fine =
        get(aml, host, dev, *b"_FIF").and_then(|o| parse_fif(&o)).is_some_and(|i| i.fine_grain);
    let states = get(aml, host, dev, *b"_FPS").and_then(|o| parse_fps(&o)).unwrap_or_default();
    levels(fine, &states).map_or(Drive::Power, |(on, off)| Drive::Level { on, off })
}
