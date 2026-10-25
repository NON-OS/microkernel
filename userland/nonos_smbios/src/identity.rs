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

//! What NONOS serves of SMBIOS: who made the machine and what it is, for
//! the inventory, the About app and driver quirk tables. From BIOS
//! Information (type 0: vendor, version, release date), System Information
//! (type 1: manufacturer, product name, version), Baseboard Information
//! (type 2: manufacturer, product) and System Enclosure (type 3: the
//! chassis type). Serial numbers, the UUID and asset tags are never read:
//! they identify one machine, not a model.

use super::chassis::chassis_name;
use super::structure::Structure;
use super::text::text;
use alloc::string::String;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Identity {
    pub vendor: String,
    pub product: String,
    pub version: String,
    pub board_vendor: String,
    pub board: String,
    pub bios_vendor: String,
    pub bios_version: String,
    pub bios_date: String,
    pub chassis: &'static str,
}

pub fn identity(structures: &[Structure<'_>]) -> Identity {
    let first = |kind| structures.iter().find(|s| s.kind == kind);
    let s = |st: Option<&Structure<'_>>, at| {
        st.and_then(|x| x.string(at)).map(text).unwrap_or_default()
    };
    let (bios, sys, board) = (first(0), first(1), first(2));
    Identity {
        vendor: s(sys, 0x04),
        product: s(sys, 0x05),
        version: s(sys, 0x06),
        board_vendor: s(board, 0x04),
        board: s(board, 0x05),
        bios_vendor: s(bios, 0x04),
        bios_version: s(bios, 0x05),
        bios_date: s(bios, 0x08),
        chassis: first(3).and_then(|c| c.byte(0x05)).map_or("Unknown", chassis_name),
    }
}
