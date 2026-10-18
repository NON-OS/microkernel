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

//! A device's state as the capsule knows it.

use crate::boot::{evaluator, Acpi};
use nonos_aml::device_power::{power_managed, psc, DState};
use nonos_aml::namespace::Id;

/// What this capsule last set; else _PSC; else D0 for a device with no
/// power control, which is always on; else unknown.
pub fn current(acpi: &mut Acpi, dev: Id) -> Option<DState> {
    if let Some(s) = acpi.dpower.state(dev) {
        return Some(s);
    }
    let Acpi { aml, host, .. } = acpi;
    psc(aml, &mut evaluator(host), dev).or((!power_managed(aml, dev)).then_some(DState::D0))
}
