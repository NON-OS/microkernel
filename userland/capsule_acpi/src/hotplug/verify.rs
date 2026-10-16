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

//! After `_EJ0`: the eject is done once `_STA` says the device is gone; if
//! it never does, what was taken out of the kernel's table is looked at
//! again, so a device firmware kept is probed back in.

use super::cover::covered;
use super::look::look;
use super::state::{Pending, Stage};
use crate::boot::{evaluator, path, Acpi};
use crate::sys::log::Line;
use nonos_aml::hotplug::present;
use nonos_aml::namespace::Id;

pub const VERIFY_STEP_MS: u64 = 200;
/// How long firmware has to make `_STA` say absent after `_EJ0`.
pub const VERIFY_MS: u64 = 5_000;

pub fn line(acpi: &Acpi, id: Id) -> Line {
    Line::new(b"hotplug ").str(path(&acpi.aml, id).as_bytes())
}

pub fn verify(acpi: &mut Acpi, id: Id, until_ms: u64, now: u64) {
    let here = present(&mut acpi.aml, &mut evaluator(&mut acpi.host), id);
    match here {
        Some(false) => {
            covered(acpi, id).into_iter().for_each(|d| acpi.hotplug.forget(d));
            acpi.hotplug.docked.retain(|d| *d != id);
            line(acpi, id).str(b": ejected").end();
        }
        _ if now >= until_ms => {
            line(acpi, id).str(b": still present after _EJ0, looked at again").end();
            look(acpi, id, true);
        }
        _ => acpi.hotplug.pending.push(Pending {
            id,
            stage: Stage::Verify { until_ms },
            next_ms: now + VERIFY_STEP_MS,
        }),
    }
}
