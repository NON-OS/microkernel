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

//! The rows of scope_table.rs with more than one entry.

use super::ids::EMMC;
use super::lpss::LPSS_I2C;
use super::rule::Match::{self, Acpi, Class, Ids};

pub(super) const INTEL: u16 = 0x8086;
pub(super) const ANY: Option<u16> = None;

/// SATA from any vendor, Intel RST RAID, and SD/eMMC hosts (prog-if 0 to 2)
/// the capsule's emmc path serves.
pub(super) const AHCI: &[Match] = &[
    Class { vendor: ANY, class: 0x01, sub: 0x06, progif: None },
    Class { vendor: Some(INTEL), class: 0x01, sub: 0x04, progif: None },
    Class { vendor: ANY, class: 0x08, sub: 0x05, progif: Some(0x00) },
    Class { vendor: ANY, class: 0x08, sub: 0x05, progif: Some(0x01) },
    Class { vendor: ANY, class: 0x08, sub: 0x05, progif: Some(0x02) },
    Ids(INTEL, EMMC),
];

/// Intel LPSS I2C by id, any other Intel serial-bus or signal-processing
/// function the capsule takes as unlisted LPSS, and the ACPI-declared I2C
/// controllers (SERIAL) and GPIO communities (GPIO_CTRL).
pub(super) const I2C_PCI: &[Match] = &[
    Ids(INTEL, LPSS_I2C),
    Class { vendor: Some(INTEL), class: 0x0C, sub: 0x80, progif: None },
    Class { vendor: Some(INTEL), class: 0x11, sub: 0x80, progif: None },
    Acpi { class: 0x0060, vendor: ANY, device: ANY },
    Acpi { class: 0x0080, vendor: ANY, device: ANY },
];

/// The synthesized PS/2 keyboard and aux port (broker/platform.rs).
pub(super) const PS2: &[Match] = &[
    Acpi { class: 0x0040, vendor: Some(1), device: Some(0x0303) },
    Acpi { class: 0x0040, vendor: Some(1), device: Some(0x0304) },
];

/// HD Audio from any vendor, and Intel's audio-class controllers.
pub(super) const HDA: &[Match] = &[
    Class { vendor: ANY, class: 0x04, sub: 0x03, progif: None },
    Class { vendor: Some(INTEL), class: 0x04, sub: 0x01, progif: None },
];
