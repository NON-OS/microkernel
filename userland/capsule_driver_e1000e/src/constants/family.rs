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

//! The MAC family a device ID belongs to. The variants are in the order of
//! Linux `enum e1000_mac_type` (hw.h), so "this family or later" is a plain
//! comparison, the way ich8lan.c writes `hw->mac.type >= e1000_pch_spt`.

use super::ids;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Family {
    I82574,
    I82583,
    PchLpt,
    PchSpt,
    PchCnp,
    PchTgp,
    PchAdp,
    PchMtp,
    PchPtp,
}

const TABLE: &[(&[u16], Family)] = &[
    (ids::I82574, Family::I82574),
    (ids::I82583, Family::I82583),
    (ids::PCH_LPT, Family::PchLpt),
    (ids::PCH_SPT, Family::PchSpt),
    (ids::PCH_CNP, Family::PchCnp),
    (ids::PCH_TGP, Family::PchTgp),
    (ids::PCH_ADP, Family::PchAdp),
    (ids::PCH_MTP, Family::PchMtp),
    (ids::PCH_PTP, Family::PchPtp),
];

impl Family {
    pub fn of(device: u16) -> Option<Family> {
        TABLE.iter().find(|(list, _)| list.contains(&device)).map(|(_, f)| *f)
    }

    /// The I217, I218 and I219: a MAC in the chipset and a PHY on the board,
    /// reached over a link that firmware may leave in SMBus mode.
    pub fn is_pch(self) -> bool {
        self >= Family::PchLpt
    }

    /// The Linux board name, as the bring-up line prints it.
    pub fn name(self) -> &'static str {
        match self {
            Family::I82574 => "82574",
            Family::I82583 => "82583",
            Family::PchLpt => "pch_lpt",
            Family::PchSpt => "pch_spt",
            Family::PchCnp => "pch_cnp",
            Family::PchTgp => "pch_tgp",
            Family::PchAdp => "pch_adp",
            Family::PchMtp => "pch_mtp",
            Family::PchPtp => "pch_ptp",
        }
    }
}
