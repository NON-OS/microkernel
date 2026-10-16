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

//! A device that is there: powered to D0 when it is off and firmware can
//! turn it on, then each function that answers handed to the kernel for
//! one probe (`MkPciProbeFunction`), and each it stood for that no longer
//! answers taken away. A device with no function, a listed controller,
//! is published as a platform record. Nothing else is woken.

use super::functions::{answering, answers, candidates};
use super::leave::depart_all;
use super::records::arrive_record;
use super::remember::remember;
use crate::boot::{evaluator, path, Acpi};
use crate::sys::log::Line;
use crate::sys::pci_probe::probe_function;
use alloc::vec::Vec;
use nonos_aml::device_power::{power_managed, reveal};
use nonos_aml::namespace::Id;

fn power_up(acpi: &mut Acpi, id: Id) {
    let cands = candidates(acpi, id);
    if cands.is_empty() || cands.iter().any(|f| answers(*f)) || !power_managed(&acpi.aml, id) {
        return;
    }
    let Acpi { aml, host, dpower, cooling, .. } = acpi;
    let mut probe = || cands.iter().any(|f| answers(*f));
    let _ = reveal(dpower, &mut cooling.pool, aml, &mut evaluator(host), id, &mut probe);
}

pub fn arrive(acpi: &mut Acpi, id: Id) {
    if candidates(acpi, id).is_empty() && acpi.hotplug.functions(id).is_empty() {
        return arrive_record(acpi, id);
    }
    power_up(acpi, id);
    let now = answering(acpi, id);
    let gone: Vec<_> =
        acpi.hotplug.functions(id).iter().filter(|f| !now.contains(f)).copied().collect();
    let left = depart_all(&acpi.hotplug, &gone);
    let mut new = 0u64;
    for f in now.iter().filter(|f| f.0 == 0) {
        new += u64::from(probe_function(f.1, f.2, f.3).is_ok());
    }
    if new + left as u64 != 0 {
        let l = Line::new(b"hotplug ").str(path(&acpi.aml, id).as_bytes());
        l.str(b": ").dec(new).str(b" arrived, ").dec(left as u64).str(b" left").end();
    }
    remember(&mut acpi.hotplug, id, now);
}
