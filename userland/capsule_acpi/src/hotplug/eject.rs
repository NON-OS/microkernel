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

//! An eject request (Notify 3, ACPI 6.5, 6.3): firmware is asked to remove
//! the device only once no driver holds anything it takes with it, the
//! device and all below it, and for a dock all that names it in `_EJD`.
//! Each function leaves the kernel's table first (`MkPciDepart`, mode
//! `Eject`, which the kernel refuses while one is held), so nothing can be
//! claimed between the check and `_EJ0`; a dock is undocked with `_DCK(0)`
//! first. If any step fails, the functions taken out are probed back in.

use super::cover::{covered, functions, restore};
use super::state::Bdf;
use crate::boot::{evaluator, Acpi};
use crate::sys::acpi_holds::claimed_by_anyone;
use crate::sys::pci_depart::{depart, Why, EBUSY};
use alloc::vec::Vec;
use nonos_aml::hotplug::{dock, eject, is_dock};
use nonos_aml::namespace::Id;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    Held(Bdf),
    Undock,
    Eject,
}

pub fn try_eject(acpi: &mut Acpi, id: Id) -> Result<(), Refused> {
    let ids = covered(acpi, id);
    let fs = functions(&acpi.hotplug, &ids);
    if let Some(f) = fs.iter().find(|f| claimed_by_anyone(**f) == Ok(true)) {
        return Err(Refused::Held(*f));
    }
    let mut out = Vec::new();
    for &f in &fs {
        match depart(f.1, f.2, f.3, Why::Eject) {
            Ok(_) => out.push(f),
            Err(EBUSY) => {
                restore(&out);
                return Err(Refused::Held(f));
            }
            Err(_) => {}
        }
    }
    let undock = match is_dock(&acpi.aml, id) {
        true => Some(dock(&mut acpi.aml, &mut evaluator(&mut acpi.host), id, false)),
        false => None,
    };
    if matches!(undock, Some(Some(Err(_) | Ok(false)))) {
        restore(&out);
        return Err(Refused::Undock);
    }
    let r = eject(&mut acpi.aml, &mut evaluator(&mut acpi.host), id);
    r.map_err(|_| {
        restore(&out);
        Refused::Eject
    })
}
