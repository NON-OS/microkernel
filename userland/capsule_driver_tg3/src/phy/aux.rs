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

//! The Broadcom shadow registers behind AUX_CTRL and MISC_SHDW
//! (tg3_phy_auxctl_read, tg3_phy_auxctl_write, tg3_phy_shdw_write).

use crate::constants::phy::*;

use super::mdio::Mdio;

impl Mdio {
    pub fn aux_read(&self, shadow: u32) -> Result<u32, &'static str> {
        let select = (shadow << MII_TG3_AUXCTL_MISC_RDSEL_SHIFT) | MII_TG3_AUXCTL_SHDWSEL_MISC;
        self.write(MII_TG3_AUX_CTRL, select)?;
        self.read(MII_TG3_AUX_CTRL)
    }

    pub fn aux_write(&self, shadow: u32, set: u32) -> Result<(), &'static str> {
        let set = if shadow == MII_TG3_AUXCTL_SHDWSEL_MISC {
            set | MII_TG3_AUXCTL_MISC_WREN
        } else {
            set
        };
        self.write(MII_TG3_AUX_CTRL, set | shadow)
    }

    /// Set `bits` in an AUX_CTRL shadow; a read that fails leaves it alone,
    /// as each of tg3's read-modify-writes there does.
    pub fn aux_set(&self, shadow: u32, bits: u32) {
        if let Ok(v) = self.aux_read(shadow) {
            let _ = self.aux_write(shadow, v | bits);
        }
    }

    pub fn shadow_write(&self, select: u32, value: u32) -> Result<(), &'static str> {
        self.write(MII_TG3_MISC_SHDW, select | value | MII_TG3_MISC_SHDW_WREN)
    }
}
