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

//! What an eject takes with it, and putting it back when the eject fails.

use super::dock::with_dock;
use super::state::{Bdf, Hotplug};
use crate::boot::Acpi;
use crate::sys::pci_probe::probe_function;
use alloc::vec::Vec;
use nonos_aml::hotplug::{is_dock, targets};
use nonos_aml::namespace::Id;

/// The devices an eject of `id` takes with it.
pub fn covered(acpi: &mut Acpi, id: Id) -> Vec<Id> {
    let mut ids = targets(&acpi.aml, id, true);
    if is_dock(&acpi.aml, id) {
        ids.extend(with_dock(acpi, id));
    }
    ids
}

pub fn functions(hp: &Hotplug, ids: &[Id]) -> Vec<Bdf> {
    let own = ids.iter().flat_map(|i| hp.functions(*i).iter().copied());
    own.flat_map(|f| hp.behind(f).into_iter().chain(core::iter::once(f)))
        .filter(|f| f.0 == 0)
        .collect()
}

pub fn restore(out: &[Bdf]) {
    for f in out {
        let _ = probe_function(f.1, f.2, f.3);
    }
}
