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

//! The link context: which PHY the station's one link runs on, whether it
//! is active, and the BSS's rates and timing.
//!
//! `struct iwl_link_config_cmd` (fw/api/mac-cfg.h),
//! LINK_CONTEXT_CONFIG_CMD_API_S_VER_1, 208 bytes: `action` 0, `link_id` 4,
//! `mac_id` 8, `phy_id` 12, `local_link_addr` 16, `modify_mask` 24, `active`
//! 28, `listen_lmac` 32, `cck_rates` 36, `ofdm_rates` 40,
//! `cck_short_preamble` 44, `short_slot` 48, `protection_flags` 52,
//! `qos_flags` 56, the five `iwl_ac_qos` at 60..100, the HE fields at
//! 100..136, `bi` 136, `dtim_interval` 140, `puncture_mask` 144,
//! `frame_time_rts_th` 146, `flags` 148, `flags_mask` 152, the multi-BSSID
//! fields at 156..176 (`spec_link_id` at 166) and reserved words to 208.
//!
//! Filled as Linux v6.12 mvm/link.c `iwl_mvm_add_link` and
//! `iwl_mvm_link_changed` fill it for link 0 of MAC 0: the add names no PHY
//! (`FW_CTXT_INVALID`); a change names the PHY, the link's state, the ACK
//! rates (`iwl_mvm_set_fw_basic_rates`), the ERP preamble and slot, and the
//! beacon interval and DTIM interval (`bi * dtim_period`), and `modify_mask`
//! says which of them the firmware is to take. `listen_lmac` stays 0 (this
//! firmware does not list the command, so version 1). A legacy, non-QoS
//! association sets no protection, QoS or HE fields: Linux sends
//! `LINK_CONTEXT_MODIFY_QOS_PARAMS` only for a QoS BSS and the HE parameters
//! only for an HE one.

use alloc::vec;
use alloc::vec::Vec;

pub const LINK_CONFIG_LEN: usize = 208;
/// The one link and its MAC.
pub const LINK_ID: u32 = 0;
pub const MAC_ID: u32 = 0;
/// `FW_CTXT_INVALID`: no PHY.
pub const CTXT_INVALID: u32 = 0xFFFF_FFFF;

/// `LINK_CONTEXT_MODIFY_ACTIVE` and `LINK_CONTEXT_MODIFY_RATES_INFO`.
pub const MODIFY_ACTIVE: u32 = 1 << 0;
pub const MODIFY_RATES_INFO: u32 = 1 << 1;

/// What one link command says.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LinkParams {
    pub addr: [u8; 6],
    pub phy: Option<u32>,
    pub active: bool,
    pub modify_mask: u32,
    pub cck_ack: u8,
    pub ofdm_ack: u8,
    pub short_preamble: bool,
    pub short_slot: bool,
    /// Beacon interval in TU, and the DTIM period in beacons.
    pub beacon_int: u16,
    pub dtim_period: u8,
}

/// `iwl_link_config_cmd` for link 0 of MAC 0.
pub fn link_config(action: u32, p: &LinkParams) -> Vec<u8> {
    let mut c = vec![0u8; LINK_CONFIG_LEN];
    let mut put = |at: usize, v: u32| c[at..at + 4].copy_from_slice(&v.to_le_bytes());
    put(0, action);
    put(4, LINK_ID);
    put(8, MAC_ID);
    put(12, p.phy.unwrap_or(CTXT_INVALID));
    put(24, p.modify_mask);
    put(28, u32::from(p.active));
    put(36, u32::from(p.cck_ack));
    put(40, u32::from(p.ofdm_ack));
    put(44, u32::from(p.short_preamble));
    put(48, u32::from(p.short_slot));
    put(136, u32::from(p.beacon_int));
    put(140, u32::from(p.beacon_int).saturating_mul(u32::from(p.dtim_period)));
    c[16..22].copy_from_slice(&p.addr);
    c[166] = LINK_ID as u8;
    c
}
