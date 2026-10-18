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

//! SMBIOS (8): the machine's identity, read-only, for the inventory, the
//! About app and driver quirk tables: vendor, product, version, board
//! vendor, board, BIOS vendor, BIOS version, BIOS date and chassis, each a
//! length byte and its text. Never a serial number or the UUID.

use super::protocol::ENOENT;
use crate::boot::Acpi;
use alloc::vec::Vec;

pub fn smbios(acpi: &Acpi) -> (i32, Vec<u8>) {
    let Some(id) = &acpi.smbios else { return (ENOENT, Vec::new()) };
    let fields = [
        id.vendor.as_bytes(),
        id.product.as_bytes(),
        id.version.as_bytes(),
        id.board_vendor.as_bytes(),
        id.board.as_bytes(),
        id.bios_vendor.as_bytes(),
        id.bios_version.as_bytes(),
        id.bios_date.as_bytes(),
        id.chassis.as_bytes(),
    ];
    let mut v = Vec::new();
    for f in fields {
        v.push(f.len() as u8);
        v.extend_from_slice(f);
    }
    (0, v)
}
