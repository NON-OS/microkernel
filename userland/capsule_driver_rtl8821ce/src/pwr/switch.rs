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

//! rtw88 `rtw_mac_power_on` (mac.c:378) for the 8821C on PCIe. A card a warm
//! reboot left powered still holds the last boot's MAC, DMA and firmware
//! state, so it is powered off and on again, the way rtw88 does it.

use super::{presys::pre_system_cfg, rpwm::wake_firmware, run_pwr_seq, sysinit::init_system_cfg};
use super::{CARD_DISABLE, CARD_EMULATE, CARD_ENABLE, CR_UNPOWERED};
use crate::constants::regs::REG_CR;
use crate::regs::Mmio;

/// The power-on outcome. A card found powered carries REG_CR as read after
/// the power-off: 0xEA means the power-off took.
pub enum PowerOn {
    FromCold,
    WasLeftPowered { cr_after_off: u8 },
    Failed,
}

/// Power the MAC on, cycling it first when a warm reboot left it powered.
pub fn power_on<M: Mmio>(mmio: &M) -> PowerOn {
    pre_system_cfg(mmio);
    wake_firmware(mmio);
    let mut cr_after_off = None;
    if mmio.read8(REG_CR) != CR_UNPOWERED {
        // rtw88 ignores the power-off result (mac.c:388) and goes on.
        wake_firmware(mmio);
        let _ = run_pwr_seq(mmio, CARD_DISABLE);
        // rtw88 gives up here when REG_CR is still not 0xEA (mac.c:394);
        // this driver records it and still tries the power-on.
        cr_after_off = Some(mmio.read8(REG_CR));
        pre_system_cfg(mmio);
        wake_firmware(mmio);
    }
    // card_enable_flow_8821c: card disable to emulation, then to active.
    if !run_pwr_seq(mmio, CARD_EMULATE) || !run_pwr_seq(mmio, CARD_ENABLE) {
        return PowerOn::Failed;
    }
    init_system_cfg(mmio);
    match cr_after_off {
        Some(cr) => PowerOn::WasLeftPowered { cr_after_off: cr },
        None => PowerOn::FromCold,
    }
}
