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

//! The words tg3 keeps of registers it rewrites piecewise: tp->mac_mode,
//! rx_mode, tx_mode, grc_local_ctrl and mi_mode, and whether ASF firmware
//! is running as the last chip reset found it.

use crate::constants::grc::{
    GRC_LCLCTRL_AUTO_SEEPROM, GRC_LCLCTRL_GPIO_OE1, GRC_LCLCTRL_GPIO_OUTPUT1,
    GRC_LCLCTRL_GPIO_UART_SEL, GRC_LCLCTRL_INT_ON_ATTN,
};
use crate::constants::mi::MAC_MI_MODE_500KHZ_CONST;

use super::identify::Chip;

#[derive(Clone, Copy, Debug)]
pub struct Modes {
    pub mac_mode: u32,
    pub rx_mode: u32,
    pub tx_mode: u32,
    pub grc_local_ctrl: u32,
    pub mi_mode: u32,
    /// ENABLE_ASF, which on a 5750_PLUS part is ASF_NEW_HANDSHAKE too.
    pub asf: bool,
    /// tp->link_up: what the last PHY setup concluded.
    pub link_up: bool,
}

impl Modes {
    /// tg3_get_invariants' starting words for the 57765 class.
    pub fn new(chip: &Chip) -> Self {
        let mut grc_local_ctrl =
            GRC_LCLCTRL_INT_ON_ATTN | GRC_LCLCTRL_AUTO_SEEPROM | GRC_LCLCTRL_GPIO_UART_SEL;
        if chip.hw.eeprom_write_protect {
            grc_local_ctrl |= GRC_LCLCTRL_GPIO_OE1 | GRC_LCLCTRL_GPIO_OUTPUT1;
        }
        Modes {
            mac_mode: 0,
            rx_mode: 0,
            tx_mode: 0,
            grc_local_ctrl,
            mi_mode: MAC_MI_MODE_500KHZ_CONST,
            asf: chip.hw.asf,
            link_up: false,
        }
    }
}
