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

//! The mode words tg3_reset_hw works out before writing them, as pure
//! functions of the chip and of what a register holds, so the proofs can
//! hold each to Linux's.

use crate::constants::blocks::*;
use crate::constants::chip::{asic_rev, ASIC_REV_57766};
use crate::constants::grc::*;

/// RDMAC_MODE: the error enables, long bursts on PCI Express, 2 KiB reads
/// for a standard MTU on the 57766, and both LSO enables a HW_TSO_3,
/// 57765_PLUS part sets though this driver sends no LSO frames.
pub fn rdmac_mode(rev: u32) -> u32 {
    let mut v = RDMAC_MODE_BASE | RDMAC_MODE_FIFO_LONG_BURST;
    if asic_rev(rev) == ASIC_REV_57766 {
        v |= RDMAC_MODE_JMB_2K_MMRR;
    }
    v | RDMAC_MODE_IPV4_LSO_EN | RDMAC_MODE_IPV6_LSO_EN
}

/// WDMAC_MODE: the error enables and the host coalescing fix (5755_PLUS).
pub fn wdmac_mode() -> u32 {
    WDMAC_MODE_BASE | WDMAC_MODE_STATUS_TAG_FIX
}

const GPIO_MASK: u32 = GRC_LCLCTRL_GPIO_OE0
    | GRC_LCLCTRL_GPIO_OE1
    | GRC_LCLCTRL_GPIO_OE2
    | GRC_LCLCTRL_GPIO_OUTPUT0
    | GRC_LCLCTRL_GPIO_OUTPUT1
    | GRC_LCLCTRL_GPIO_OUTPUT2;

/// GRC_LOCAL_CTRL. On a LOM (not IS_NIC) the bootcode set the GPIOs and
/// they are kept as read, with GPIO1 driven high for the write protect.
pub fn grc_local_ctrl(word: u32, write_protect: bool, current: u32) -> u32 {
    if !write_protect {
        return word;
    }
    (word & !GPIO_MASK) | (current & GPIO_MASK) | GRC_LCLCTRL_GPIO_OE1 | GRC_LCLCTRL_GPIO_OUTPUT1
}
