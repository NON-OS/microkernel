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

//! The rows of scope_table.rs for the SD, xHCI and SPI hosts, each with
//! more than one entry.

use super::ids::EMMC;
use super::rows::{ANY, INTEL};
use super::rule::Match::{self, Acpi, Class, Fdt, Ids};

/// SD/eMMC hosts (prog-if 0 to 2), the Intel eMMC hosts by id, and the SD
/// hosts firmware names (acpi/platform_record.rs: broker class BLOCK).
pub(super) const SDHCI: &[Match] = &[
    Class { vendor: ANY, class: 0x08, sub: 0x05, progif: Some(0x00) },
    Class { vendor: ANY, class: 0x08, sub: 0x05, progif: Some(0x01) },
    Class { vendor: ANY, class: 0x08, sub: 0x05, progif: Some(0x02) },
    Ids(INTEL, EMMC),
    Acpi { class: 0x0010, vendor: ANY, device: ANY },
];

/// xHCI from any vendor, and the device tree's xHCI (interface 2).
pub(super) const XHCI: &[Match] =
    &[Class { vendor: ANY, class: 0x0C, sub: 0x03, progif: Some(0x30) }, Fdt(2)];

/// The Intel serial-bus and signal-processing functions an LPSS SPI host
/// may be; the capsule proves which after the claim.
pub(super) const SPI: &[Match] = &[
    Class { vendor: Some(INTEL), class: 0x0C, sub: 0x80, progif: None },
    Class { vendor: Some(INTEL), class: 0x11, sub: 0x80, progif: None },
];
