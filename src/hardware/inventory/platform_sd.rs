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

//! An SD host the ACPI capsule published as a platform record: no PCI
//! function, the record's bus kind ACPI, the SD host class (08h, 05h).
//! It is an eMMC host or an SD card host (80860F14 and AMDI0040 are
//! eMMC, 80860F16 an SD slot); the record does not say which, and its
//! driver reads the slot type from the controller itself. Pure, with no
//! imports, so host proofs can include it by path.

/// The record's bus kind for a device the ACPI capsule published.
pub(crate) const BUS_ACPI: u8 = 2;
const PCI_CLASS_SYSTEM: u8 = 0x08;
const PCI_SUBCLASS_SDHCI: u8 = 0x05;

pub(crate) const fn is_platform_sd_host(bus_kind: u8, class: u8, subclass: u8) -> bool {
    bus_kind == BUS_ACPI && class == PCI_CLASS_SYSTEM && subclass == PCI_SUBCLASS_SDHCI
}
