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

//! The board facts the bring-up follows.

use crate::constants::mi::LED_CTRL_MODE_PHY_1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HwCfg {
    /// ASF firmware runs beside the driver and takes the handshake.
    pub asf: bool,
    /// EEPROM_WRITE_PROT: GPIO1 is driven high. Its absence is IS_NIC.
    pub eeprom_write_protect: bool,
    pub led_ctrl: u32,
    /// TG3_PHYFLG_ENABLE_APD, the PHY's auto power-down.
    pub apd: bool,
    /// TG3_PHYFLG_KEEP_LINK_ON_PWRDN: the PHY is not reset at open.
    pub keep_link: bool,
    /// TG3_PHYFLG_CAPACITIVE_COUPLING: one more relink step before autoneg.
    pub capacitive_coupling: bool,
}

/// What tg3_get_eeprom_hw_cfg assumes when the signature is missing.
pub const NO_SIGNATURE: HwCfg = HwCfg {
    asf: false,
    eeprom_write_protect: true,
    led_ctrl: LED_CTRL_MODE_PHY_1,
    apd: false,
    keep_link: false,
    capacitive_coupling: false,
};
