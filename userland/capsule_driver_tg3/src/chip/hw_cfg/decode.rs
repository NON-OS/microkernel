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

//! The SRAM words decoded as tg3_get_eeprom_hw_cfg does for a 5750_PLUS,
//! PCI Express part. Pure, so tg3_proofs feeds it every word a hostile part
//! could leave.

use crate::constants::sram::*;

use super::cfg::{HwCfg, NO_SIGNATURE};
use super::led::led;

/// `words` is the signature, NIC_SRAM_DATA_CFG, _VER, _CFG_2 and _CFG_3.
pub fn decode(words: [u32; 5]) -> Result<HwCfg, &'static str> {
    let [sig, nic_cfg, ver, cfg2, cfg3] = words;
    if sig != NIC_SRAM_DATA_SIG_MAGIC {
        return Ok(NO_SIGNATURE);
    }
    if nic_cfg & NIC_SRAM_DATA_CFG_PHY_TYPE_MASK == NIC_SRAM_DATA_CFG_PHY_TYPE_FIBER {
        return Err("fiber port; this driver drives copper");
    }
    // tg3 maps BAR2 for the APE only by device ID, which no 57765-class part
    // is on; a board that turns it on anyway is refused, not half driven.
    if nic_cfg & NIC_SRAM_DATA_CFG_APE_ENABLE != 0 {
        return Err("APE management firmware enabled; not driven");
    }
    let ver = ver >> NIC_SRAM_DATA_VER_SHIFT;
    let cfg2 = if ver > 0 && ver < 0x100 { cfg2 } else { 0 };
    Ok(HwCfg {
        asf: nic_cfg & NIC_SRAM_DATA_CFG_ASF_ENABLE != 0,
        eeprom_write_protect: nic_cfg & NIC_SRAM_DATA_CFG_EEPROM_WP != 0,
        led_ctrl: led(cfg2 & (NIC_SRAM_DATA_CFG_LED_MODE_MASK | SHASTA_EXT_LED_MODE_MASK)),
        apd: cfg2 & NIC_SRAM_DATA_CFG_2_APD_EN != 0,
        keep_link: cfg3 & NIC_SRAM_LNK_FLAP_AVOID != 0,
        capacitive_coupling: cfg2 & NIC_SRAM_DATA_CFG_2_CAPACITIVE != 0,
    })
}
