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

//! Taking an eject's devices out of the kernel's table before firmware is
//! asked, and putting them back if it refuses: each function and platform
//! record in eject mode, which the kernel refuses while one is held.

use super::cover::{functions, restore};
use super::eject::Refused;
use super::records::{put_back, records};
use super::state::Bdf;
use crate::boot::Acpi;
use crate::sys::acpi_holds::claimed_by_anyone;
use crate::sys::pci_depart::{depart, depart_record, Why, EBUSY};
use alloc::vec::Vec;
use nonos_aml::namespace::Id;

/// What an eject has taken out of the table, to put back if it fails.
pub struct Taken {
    functions: Vec<Bdf>,
    records: Vec<Id>,
}

pub fn put_back_all(acpi: &mut Acpi, t: &Taken) {
    restore(&t.functions);
    put_back(acpi, &t.records);
}

pub fn take_out(acpi: &mut Acpi, ids: &[Id]) -> Result<Taken, Refused> {
    let fs = functions(&acpi.hotplug, ids);
    if let Some(f) = fs.iter().find(|f| claimed_by_anyone(**f) == Ok(true)) {
        return Err(Refused::Held(*f));
    }
    let mut t = Taken { functions: Vec::new(), records: Vec::new() };
    for &f in &fs {
        match depart(f.1, f.2, f.3, Why::Eject) {
            Ok(_) => t.functions.push(f),
            Err(EBUSY) => {
                put_back_all(acpi, &t);
                return Err(Refused::Held(f));
            }
            Err(_) => {}
        }
    }
    for (id, r) in records(acpi, ids) {
        match depart_record(r, Why::Eject) {
            Ok(_) => t.records.push(id),
            Err(EBUSY) => {
                put_back_all(acpi, &t);
                return Err(Refused::HeldRecord(r));
            }
            Err(_) => {}
        }
    }
    Ok(t)
}
