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

//! Which moves a PCI function's configuration does not survive, so that
//! DEVICE_POWER refuses them until the kernel can put it back (PCI Power
//! Management, as recalled, unverified): D3cold, whose power goes, and
//! D3hot unless the function's power management status says No_Soft_Reset
//! (bit 3), without which leaving D3hot resets it. A function with no
//! power management capability cannot say so, and is refused D3hot too.
//! D1 and D2 keep it. Staying where it is changes nothing.

use super::state::DState;

pub const NO_SOFT_RESET: u16 = 1 << 3;

/// `pmcsr`: the function's power management control and status register,
/// None when it has no power management capability.
pub fn loses_config(from: DState, to: DState, pmcsr: Option<u16>) -> bool {
    if from == to {
        return false;
    }
    match to {
        DState::D3Cold => true,
        DState::D3Hot => pmcsr.is_none_or(|p| p & NO_SOFT_RESET == 0),
        _ => false,
    }
}
