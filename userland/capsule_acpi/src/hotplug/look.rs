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

//! A bus check, device check or device check light (ACPI 6.5, 5.6.6):
//! each device it covers, parents first, is there or gone by its `_STA`.
//! Below a device that is gone nothing is asked: it went with its parent,
//! whatever its own `_STA` would say. A dock found present is docked with
//! `_DCK(1)` before anything below it is looked at.

use super::arrive::arrive;
use super::dock::{docked, undocked};
use super::leave::leave;
use crate::boot::{evaluator, Acpi};
use alloc::vec::Vec;
use nonos_aml::hotplug::{is_dock, present, targets};
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;

fn under(aml: &Aml, mut t: Id, a: Id) -> bool {
    for _ in 0..64 {
        let Some(p) = aml.ns.get(t).map(|e| e.parent).filter(|p| *p != t) else { return false };
        if p == a {
            return true;
        }
        t = p;
    }
    false
}

pub fn look(acpi: &mut Acpi, id: Id, deep: bool) {
    if is_dock(&acpi.aml, id) {
        let here = present(&mut acpi.aml, &mut evaluator(&mut acpi.host), id);
        match here {
            Some(true) if !docked(acpi, id) => return,
            Some(false) => undocked(acpi, id),
            _ => {}
        }
    }
    let mut gone: Vec<Id> = Vec::new();
    for t in targets(&acpi.aml, id, deep) {
        let here = match gone.iter().any(|g| under(&acpi.aml, t, *g)) {
            true => Some(false),
            false => present(&mut acpi.aml, &mut evaluator(&mut acpi.host), t),
        };
        match here {
            Some(true) => arrive(acpi, t),
            Some(false) => {
                gone.push(t);
                leave(acpi, t);
            }
            None => {}
        }
    }
}
