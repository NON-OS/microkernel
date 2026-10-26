// NONOS Operating System (AGPL-3.0-or-later)

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

mod chains;
mod context;

pub use chains::*;
pub use context::*;
