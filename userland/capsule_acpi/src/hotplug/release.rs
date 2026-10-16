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

//! Firmware's side of an eject, run through `nonos_aml::hotplug::release`
//! (unlock, undock, `_EJ0`, locked again on a failure after the unlock),
//! and the lock taken when a dock docks or a lockable device arrives, so
//! it cannot be pulled while in use.

use super::verify::line;
use crate::boot::{evaluator, Acpi};
use nonos_aml::hotplug::{dock, eject, ejectable, is_dock, lock, lockable, removable};
use nonos_aml::hotplug::{release, Has, Released, Steps, Stopped};
use nonos_aml::namespace::Id;

struct Firmware<'a> {
    acpi: &'a mut Acpi,
    id: Id,
}

impl Steps for Firmware<'_> {
    fn lock(&mut self, on: bool) -> bool {
        let r = lock(&mut self.acpi.aml, &mut evaluator(&mut self.acpi.host), self.id, on);
        let ok = matches!(r, Some(Ok(())));
        if !ok && on {
            line(self.acpi, self.id).str(b": _LCK(1) failed, left unlocked").end();
        }
        ok
    }
    fn undock(&mut self) -> bool {
        let r = dock(&mut self.acpi.aml, &mut evaluator(&mut self.acpi.host), self.id, false);
        matches!(r, Some(Ok(true)))
    }
    fn eject(&mut self) -> bool {
        eject(&mut self.acpi.aml, &mut evaluator(&mut self.acpi.host), self.id).is_ok()
    }
}

/// Whether an eject may be asked of the device at all.
pub fn may_leave(acpi: &mut Acpi, id: Id) -> bool {
    ejectable(&acpi.aml, id) || removable(&mut acpi.aml, &mut evaluator(&mut acpi.host), id)
}

pub fn let_go(acpi: &mut Acpi, id: Id) -> Result<Released, Stopped> {
    let h = Has {
        ej0: ejectable(&acpi.aml, id),
        removable: removable(&mut acpi.aml, &mut evaluator(&mut acpi.host), id),
        lck: lockable(&acpi.aml, id),
        dock: is_dock(&acpi.aml, id),
    };
    release(&mut Firmware { acpi, id }, h)
}

/// `_LCK(1)` for a device that has one; said when it fails.
pub fn lock_in(acpi: &mut Acpi, id: Id) {
    if lockable(&acpi.aml, id) {
        let _ = Firmware { acpi, id }.lock(true);
    }
}
