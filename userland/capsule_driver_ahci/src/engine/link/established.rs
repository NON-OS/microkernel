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

use crate::constants::regs::{SSTS_DET_DETECTING, SSTS_DET_MASK, SSTS_DET_PRESENT};

/// True once PxSSTS.DET reports a present device with PHY communication up (3h).
/// Firmware may hand the port over before this is set, so init must wait for it
/// rather than issue a command into a port whose link is not yet established.
pub fn link_established(ssts: u32) -> bool {
    ssts & SSTS_DET_MASK == SSTS_DET_PRESENT
}

/// Where a link stands after COMRESET, by PxSSTS.DET.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    /// DET = 3: a device, and the PHY talks to it.
    Up,
    /// DET = 1: a device was seen and the PHY is still negotiating.
    Negotiating,
    /// DET = 0 (nothing seen) or 4 (PHY offline), or a reserved value: no
    /// device answers. Only a reading that holds for a while means an empty
    /// port; right after COMRESET a present device also reads 0 at first.
    Quiet,
}

pub fn link_state(ssts: u32) -> LinkState {
    if link_established(ssts) {
        LinkState::Up
    } else if ssts & SSTS_DET_MASK == SSTS_DET_DETECTING {
        LinkState::Negotiating
    } else {
        LinkState::Quiet
    }
}
