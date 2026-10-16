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

//! A device that is gone: every function it stood for, and every function
//! on the buses behind any bridge among them, taken out of the kernel's
//! table. The kernel revokes what each one's holder was given before it
//! answers (`MkPciDepart`, mode `Gone`).

use super::state::{Bdf, Hotplug};
use crate::boot::{path, Acpi};
use crate::sys::log::Line;
use crate::sys::pci_depart::{depart, Why};
use nonos_aml::namespace::Id;

/// How many of `bdfs`, and of what is behind them, the kernel had.
pub fn depart_all(hp: &Hotplug, bdfs: &[Bdf]) -> usize {
    let mut n = 0;
    for &f in bdfs {
        for g in hp.behind(f).into_iter().chain(core::iter::once(f)).filter(|g| g.0 == 0) {
            n += usize::from(depart(g.1, g.2, g.3, Why::Gone).is_ok());
        }
    }
    n
}

pub fn leave(acpi: &mut Acpi, id: Id) {
    let bdfs = acpi.hotplug.functions(id).to_vec();
    let n = depart_all(&acpi.hotplug, &bdfs);
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
