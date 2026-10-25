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

//! What Arm has published about its own cores, by MIDR part number, for the
//! cases the ID registers cannot answer. Recalled from Arm's speculation
//! security updates (the Spectre/Meltdown core table, and the Spectre-BHB
//! guidance of March 2022), not from a copy in this tree: every entry is
//! listed in the letter as unverified for the owner.

/// MIDR_EL1.Implementer for Arm Limited.
const ARM: u8 = 0x41;

const A35: u16 = 0xD04;
const A53: u16 = 0xD03;
const A55: u16 = 0xD05;
const A57: u16 = 0xD07;
const A72: u16 = 0xD08;
const A75: u16 = 0xD0A;
const A76: u16 = 0xD0B;
const N1: u16 = 0xD0C;
const A77: u16 = 0xD0D;
const V1: u16 = 0xD40;
const A78: u16 = 0xD41;
const A78AE: u16 = 0xD42;
const X1: u16 = 0xD44;
const A710: u16 = 0xD47;
const X2: u16 = 0xD48;
const N2: u16 = 0xD49;
const A78C: u16 = 0xD4B;

/// An Arm core.
pub(crate) const fn is_arm(implementer: u8) -> bool {
    implementer == ARM
}

/// The in-order cores Arm lists as unaffected by Spectre variant 2,
/// speculative store bypass and Spectre-BHB.
pub(crate) const fn in_order(implementer: u8, part: u16) -> bool {
    is_arm(implementer) && matches!(part, A35 | A53 | A55)
}

/// The one Arm core listed as affected by Meltdown (variant 3).
pub(crate) const fn meltdown(implementer: u8, part: u16) -> bool {
    is_arm(implementer) && part == A75
}

/// The branch count `k` of the history-clearing loop for this core, where
/// Arm gives one.
pub(crate) const fn bhb_loop(implementer: u8, part: u16) -> Option<u8> {
    if !is_arm(implementer) {
        return None;
    }
    match part {
        A57 | A72 => Some(8),
        A76 | A77 | N1 => Some(24),
        A78 | A78AE | A78C | X1 | X2 | A710 | N2 | V1 => Some(32),
        _ => None,
    }
}
