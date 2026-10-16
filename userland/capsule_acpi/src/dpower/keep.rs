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
//! would not survive them (`nonos_aml::device_power::loses_config_claimed`).
//! A move is made only for the function's claim holder (`target`, asked as
//! `Asker::Holder`), so the function is claimed, and the broker puts its
//! BARs back once it is in D0 again (L62): D3hot is allowed with or without
//! No_Soft_Reset. D3cold is refused before anything runs (ELOSES) until
//! the bridge and the link are handled too. Decoding and bus mastering are
//! the driver's to turn back on after a reset, as after its claim.

use super::ready::pci_function;
use crate::boot::Acpi;
use nonos_aml::device_power::{loses_config_claimed, DState};
use nonos_aml::namespace::Id;

/// Whether `dev` may move `from` -> `to` without losing what it holds.
pub fn keeps(acpi: &mut Acpi, dev: Id, from: DState, to: DState) -> bool {
    pci_function(acpi, dev).is_none() || !loses_config_claimed(from, to)
}
