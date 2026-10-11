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

//! MAC_LED_CTRL from the board's LED mode bits: the 5750_PLUS reading of
//! cfg2, for a part that is neither a 5750 A0/A1 nor 5717_PLUS or 5762.

use crate::constants::mi::{
    LED_CTRL_MODE_COMBO, LED_CTRL_MODE_MAC, LED_CTRL_MODE_PHY_1, LED_CTRL_MODE_PHY_2,
    LED_CTRL_MODE_SHARED, LED_CTRL_MODE_SHASTA_MAC,
};
use crate::constants::sram::{
    NIC_SRAM_DATA_CFG_LED_MODE_MAC, NIC_SRAM_DATA_CFG_LED_MODE_PHY_2, SHASTA_EXT_LED_COMBO,
    SHASTA_EXT_LED_MAC, SHASTA_EXT_LED_SHARED,
};

const BOTH_PHY: u32 = LED_CTRL_MODE_PHY_1 | LED_CTRL_MODE_PHY_2;

pub fn led(cfg: u32) -> u32 {
    match cfg {
        NIC_SRAM_DATA_CFG_LED_MODE_PHY_2 => LED_CTRL_MODE_PHY_2,
        NIC_SRAM_DATA_CFG_LED_MODE_MAC => LED_CTRL_MODE_MAC,
        SHASTA_EXT_LED_SHARED => LED_CTRL_MODE_SHARED | BOTH_PHY,
        SHASTA_EXT_LED_MAC => LED_CTRL_MODE_SHASTA_MAC,
        SHASTA_EXT_LED_COMBO => LED_CTRL_MODE_COMBO | BOTH_PHY,
        // NIC_SRAM_DATA_CFG_LED_MODE_PHY_1 and anything else.
        _ => LED_CTRL_MODE_PHY_1,
    }
}
