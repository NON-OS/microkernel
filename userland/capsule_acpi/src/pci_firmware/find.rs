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

//! At start: every host bridge's facts, then every device below one that
//! firmware labels (function 7), by the PCI function its `_ADR` resolves
//! to. Each is logged once.

use super::hosts::hosts;
use super::say::{say_host, say_label};
use super::state::Labelled;
use crate::boot::{evaluator, Acpi};
use crate::dpower::bridge_bus;
use alloc::vec::Vec;
use nonos_aml::pci_dsm::device_label;
use nonos_aml::platform::{device_pci_path, function_of};

pub fn find(acpi: &mut Acpi) {
    let hs = hosts(acpi);
    hs.iter().for_each(|h| say_host(&acpi.aml, h));
    acpi.pci_firmware.hosts = hs;
    let ids: Vec<_> = acpi.devices.iter().filter(|d| d.adr.is_some()).map(|d| d.id).collect();
    for id in ids {
        let eval = &mut evaluator(&mut acpi.host);
        let Some(label) = device_label(&mut acpi.aml, eval, id) else { continue };
        let Some(path) = device_pci_path(&mut acpi.aml, eval, id) else { continue };
        let Some(bdf) = function_of(&path, &mut bridge_bus) else { continue };
        say_label(&acpi.aml, id, bdf, &label);
        acpi.pci_firmware.labels.push(Labelled { bdf, label });
    }
}
