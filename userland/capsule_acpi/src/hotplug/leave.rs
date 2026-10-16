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

//! A device that is gone: every function it stood for, every function on
//! the buses behind any bridge among them, and its platform record if it
//! has one, taken out of the kernel's table. The kernel revokes what each one's holder was given before it
//! answers (`MkPciDepart`, mode `Gone`).

use super::records::depart_gone;
use super::state::Bdf;
use alloc::vec::Vec;
use crate::boot::{path, Acpi};
use crate::sys::log::Line;
use crate::sys::pci_depart::{depart, Why};
use nonos_aml::namespace::Id;

/// The device ids the kernel had among `bdfs` and what is behind them,
/// each now gone; their runtime wake is dropped at once.
pub fn depart_all(acpi: &mut Acpi, bdfs: &[Bdf]) -> usize {
    let mut ids = Vec::new();
    for &f in bdfs {
        let all = acpi.hotplug.behind(f).into_iter().chain(core::iter::once(f));
        for g in all.filter(|g| g.0 == 0) {
            ids.extend(depart(g.1, g.2, g.3, Why::Gone).ok());
        }
    }
    crate::dpower::departed(acpi, &ids);
    ids.len()
}

pub fn leave(acpi: &mut Acpi, id: Id) {
    let bdfs = acpi.hotplug.functions(id).to_vec();
    let record: Vec<u64> = depart_gone(acpi, id).into_iter().collect();
    crate::dpower::departed(acpi, &record);
    let n = depart_all(acpi, &bdfs) + record.len();
    acpi.hotplug.forget(id);
    if n != 0 {
        Line::new(b"hotplug ")
            .str(path(&acpi.aml, id).as_bytes())
            .str(b": ")
            .dec(n as u64)
            .str(b" functions left")
            .end();
    }
}
