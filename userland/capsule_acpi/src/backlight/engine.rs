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

//! Whether a display engine holds an output's GPU: the output's parent, the
//! graphics adapter, as a PCI function, asked of the broker with the claim
//! test every holder question uses. None when either cannot be told. And
//! so whether firmware's _BCM may drive the panels a request names.

use super::engine_rule::bcm_allowed;
use crate::boot::Acpi;
use crate::sys::acpi_holds::claimed_by_anyone;
use alloc::vec::Vec;
use nonos_aml::namespace::Id;

fn gpu_claimed(acpi: &mut Acpi, output: Id) -> Option<bool> {
    let adapter = acpi.aml.ns.get(output).map(|e| e.parent)?;
    let f = crate::dpower::pci_function(acpi, adapter)?;
    claimed_by_anyone(f).ok()
}

/// Every output `panel` names (all when None) may take _BCM: no display
/// engine holds its GPU (F-96).
pub fn firmware_drives(acpi: &mut Acpi, panel: Option<u32>) -> bool {
    let outputs = acpi.backlight.outputs.iter();
    let ids: Vec<Id> =
        outputs.filter(|o| panel.is_none_or(|p| p == o.panel)).map(|o| o.id).collect();
    ids.into_iter().all(|id| bcm_allowed(gpu_claimed(acpi, id)))
}
