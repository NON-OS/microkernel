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

//! The PCI functions a namespace device stands for, read while it is
//! there: its `_ADR` down the path from the host bridge (`function_of`),
//! every function when the `_ADR` names them all (0xFFFF), and only those
//! that answer.

use super::state::Bdf;
use crate::boot::{evaluator, Acpi};
use crate::dpower::bridge_bus;
use crate::sys::acpi_pci::pci_read;
use alloc::vec::Vec;
use nonos_aml::namespace::Id;
use nonos_aml::platform::{device_pci_path, function_of};

const ALL_FUNCTIONS: u32 = 0xFFFF;
const NOTHING: u32 = 0xFFFF;

pub fn answers(f: Bdf) -> bool {
    pci_read(f, 0, 2).is_ok_and(|v| v != NOTHING)
}

/// Every function `id`'s `_ADR` names, answering or not; empty when it
/// has no path to a host bridge or a bridge above it cannot be read.
pub fn candidates(acpi: &mut Acpi, id: Id) -> Vec<Bdf> {
    let Some(path) = device_pci_path(&mut acpi.aml, &mut evaluator(&mut acpi.host), id) else {
        return Vec::new();
    };
    let Some(&last) = path.adrs.last() else { return Vec::new() };
    let adrs: Vec<u32> = match last & 0xFFFF {
        ALL_FUNCTIONS => (0..8).map(|f| last & 0xFFFF_0000 | f).collect(),
        _ => alloc::vec![last],
    };
    let mut bridge = bridge_bus;
    let each = adrs.into_iter().filter_map(|adr| {
        let mut p = path.clone();
        *p.adrs.last_mut()? = adr;
        function_of(&p, &mut bridge)
    });
    each.collect()
}

pub fn answering(acpi: &mut Acpi, id: Id) -> Vec<Bdf> {
    candidates(acpi, id).into_iter().filter(|f| answers(*f)).collect()
}
