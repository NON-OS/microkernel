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

//! The moves DEVICE_POWER refuses a PCI function because its configuration
//! would not survive them (`nonos_aml::device_power::loses_config`): the
//! kernel cannot yet put a function's state back, so D3cold, and D3hot
//! without No_Soft_Reset, are refused before anything runs (ELOSES). The
//! power management status is read from the function's own capability
//! (PCI Power Management: id 1, the register at +4; recalled).

use super::ready::pci_function;
use crate::boot::Acpi;
use crate::hotplug::native::{config_read, find_cap};
use nonos_aml::device_power::{loses_config, DState};
use nonos_aml::namespace::Id;

const PM_CAP: u8 = 0x01;
const PMCSR: u16 = 4;

/// Whether `dev` may move `from` -> `to` without losing what it holds.
pub fn keeps(acpi: &mut Acpi, dev: Id, from: DState, to: DState) -> bool {
    let Some(f) = pci_function(acpi, dev) else { return true };
    let pmcsr = find_cap(f, PM_CAP).map(|c| config_read(f, c + PMCSR, 2) as u16);
    !loses_config(from, to, pmcsr)
}
