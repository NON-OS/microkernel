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
//! Each function and platform record leaves the kernel's table first
//! (`MkPciDepart` in eject mode, refused while one is held), so nothing can
//! be claimed between the check and `_EJ0`; a dock is undocked with
//! `_DCK(0)` first. If any step fails, what was taken out is put back.

use super::cover::covered;
use super::state::Bdf;
use super::take_out::{put_back_all, take_out};
use crate::boot::{evaluator, Acpi};
use nonos_aml::hotplug::{dock, eject, is_dock};
use nonos_aml::namespace::Id;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    Held(Bdf),
    /// A platform record's holder, by the record's device id.
    HeldRecord(u64),
    Undock,
    Eject,
}

pub fn try_eject(acpi: &mut Acpi, id: Id) -> Result<(), Refused> {
    let ids = covered(acpi, id);
    let t = take_out(acpi, &ids)?;
    let undock = match is_dock(&acpi.aml, id) {
        true => Some(dock(&mut acpi.aml, &mut evaluator(&mut acpi.host), id, false)),
        false => None,
    };
    if matches!(undock, Some(Some(Err(_) | Ok(false)))) {
        put_back_all(acpi, &t);
        return Err(Refused::Undock);
    }
    let r = eject(&mut acpi.aml, &mut evaluator(&mut acpi.host), id);
    r.map_err(|_| {
        put_back_all(acpi, &t);
        Refused::Eject
    })
}
