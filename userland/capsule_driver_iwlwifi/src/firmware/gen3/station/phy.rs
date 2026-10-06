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

//! The PHY context: the channel the link runs on.
//!
//! `struct iwl_phy_context_cmd` (fw/api/phy-ctxt.h), PHY_CONTEXT_CMD_API_VE_4,
//! 32 bytes: `id_and_color` 0, `action` 4, the channel info at 8 in its
//! version 2 form (`iwl_fw_channel_info`: `channel` u32 at 8, `band` 12,
//! `width` 13, `ctrl_pos` 14, reserved 15; this firmware sets
//! `IWL_UCODE_TLV_CAPA_ULTRA_HB_CHANNELS`), `lmac_id` 16, the receive chain
//! word 20, `dsp_cfg_flags` 24, reserved 28. Filled as Linux v6.12
//! mvm/phy-ctxt.c `iwl_mvm_phy_ctxt_cmd_hdr` and `iwl_mvm_phy_ctxt_cmd_data`
//! fill it for a 20 MHz channel: PHY 0 with color 0, the band (`PHY_BAND_24`
//! 1, `PHY_BAND_5` 0), `IWL_PHY_CHANNEL_MODE20` 0, control position 0 (no
//! secondary channel), LMAC 0 (`iwl_mvm_get_lmac_id` without the CDB
//! capability, which this firmware lacks), and the receive chain word zero,
//! because with RLC_CONFIG_CMD version 2 the chains go in that command.
//! A removal carries the same channel, as `iwl_mvm_phy_ctxt_unref` sends it.
//!
//! `struct iwl_rlc_config_cmd` (fw/api/datapath.h), RLC_CONFIG_CMD_API_S_VER_2,
//! 32 bytes: `phy_id` 0, `rlc.rx_chain_info` 4, `rlc.reserved` 8, the SAD
//! properties 12..28 (left zero), `flags` 28, reserved 29..32. The chain
//! word is `iwl_mvm_phy_ctxt_set_rxchain`'s: the valid receive antennas at
//! `PHY_RX_CHAIN_VALID_POS` (1), the idle count at `PHY_RX_CHAIN_CNT_POS` (10)
//! and the active count at `PHY_RX_CHAIN_MIMO_CNT_POS` (12). A legacy
//! association asks for one static and one dynamic chain; with two or more
//! receive antennas and the power scheme off (CAM, the scheme this driver's
//! POWER_TABLE_CMD selects) Linux raises both counts to two for diversity.

/// `FW_CTXT_ACTION_ADD`, `_MODIFY`, `_REMOVE` (fw/api/mac.h).
pub const ACTION_ADD: u32 = 1;
pub const ACTION_MODIFY: u32 = 2;
pub const ACTION_REMOVE: u32 = 3;

/// The size of each command.
pub const PHY_CONTEXT_LEN: usize = 32;
pub const RLC_CONFIG_LEN: usize = 32;

/// The one PHY context this driver uses.
pub const PHY_ID: u32 = 0;

const PHY_BAND_5: u8 = 0;
const PHY_BAND_24: u8 = 1;
const PHY_CHANNEL_MODE20: u8 = 0;
const RX_CHAIN_VALID_POS: u32 = 1;
const RX_CHAIN_CNT_POS: u32 = 10;
const RX_CHAIN_MIMO_CNT_POS: u32 = 12;

/// The band a 2.4 or 5 GHz channel number is in.
pub fn band(channel: u8) -> u8 {
    if channel <= 14 {
        PHY_BAND_24
    } else {
        PHY_BAND_5
    }
}

/// `iwl_phy_context_cmd` for PHY 0 on `channel` at 20 MHz.
pub fn phy_context(action: u32, channel: u8) -> [u8; PHY_CONTEXT_LEN] {
    let mut c = [0u8; PHY_CONTEXT_LEN];
    c[0..4].copy_from_slice(&PHY_ID.to_le_bytes());
    c[4..8].copy_from_slice(&action.to_le_bytes());
    c[8..12].copy_from_slice(&u32::from(channel).to_le_bytes());
    c[12] = band(channel);
    c[13] = PHY_CHANNEL_MODE20;
    c
}

/// The receive chain word for `valid_rx_ant` (a bitmap of antennas).
pub fn rx_chain_info(valid_rx_ant: u8) -> u32 {
    let chains: u32 = if valid_rx_ant.count_ones() >= 2 { 2 } else { 1 };
    (u32::from(valid_rx_ant & 0x7) << RX_CHAIN_VALID_POS)
        | (chains << RX_CHAIN_CNT_POS)
        | (chains << RX_CHAIN_MIMO_CNT_POS)
}

/// `iwl_rlc_config_cmd` for PHY 0.
pub fn rlc_config(valid_rx_ant: u8) -> [u8; RLC_CONFIG_LEN] {
    let mut c = [0u8; RLC_CONFIG_LEN];
    c[0..4].copy_from_slice(&PHY_ID.to_le_bytes());
    c[4..8].copy_from_slice(&rx_chain_info(valid_rx_ant).to_le_bytes());
    c
}
