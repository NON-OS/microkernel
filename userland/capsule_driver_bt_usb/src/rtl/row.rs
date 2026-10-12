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

//! How a row of the chip table is written: the ROM's LMP subversion, HCI
//! revision and HCI version as it answers them, then its project and its
//! files.

use super::chip::{Chip, Format};
use super::pin::Pin;

/// A first-layout chip, with its config and whether it starts without it.
pub const fn usb(
    name: &'static str,
    rom: (u16, u16, u8),
    project: u8,
    firmware: &'static Pin,
    config: &'static Pin,
    config_needed: bool,
) -> Chip {
    row(name, rom, project, Format::Realtech, firmware, Some(config), config_needed)
}

/// A second-layout chip; linux-firmware has no USB config for any of them.
pub const fn rtbt(name: &'static str, rom: (u16, u16, u8), project: u8, fw: &'static Pin) -> Chip {
    row(name, rom, project, Format::RtbtCore, fw, None, false)
}

const fn row(
    name: &'static str,
    (lmp_subversion, hci_revision, hci_version): (u16, u16, u8),
    project: u8,
    format: Format,
    firmware: &'static Pin,
    config: Option<&'static Pin>,
    config_needed: bool,
) -> Chip {
    Chip {
        name,
        lmp_subversion,
        hci_revision,
        hci_version,
        project,
        format,
        firmware,
        config,
        config_needed,
    }
}
