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

//! The station MAC context, associated or not.
//!
//! `struct iwl_mac_config_cmd` (fw/api/mac-cfg.h),
//! MAC_CONTEXT_CONFIG_CMD_API_S_VER_2, 52 bytes: `id_and_color` 0, `action`
//! 4, `mac_type` 8, `local_mld_addr` 12, `filter_flags` 20, `he_support` 24,
//! `he_ap_support` 26, `eht_support` 28, `nic_not_ack_enabled` 32, then the
//! client data (`iwl_mac_client_data`, MAC_CONTEXT_CONFIG_CLIENT_DATA_API_S_VER_2)
//! at 36: `is_assoc` 36, `esr_transition_timeout` 37, `medium_sync_delay`
//! 38, `assoc_id` 40, `data_policy` 44, `ctwin` 48.
//!
//! Filled as Linux v6.12 mvm/mld-mac.c `iwl_mvm_mld_mac_ctxt_cmd_sta` fills
//! it for MAC 0 (`FW_MAC_TYPE_BSS_STA` 5): group frames always accepted
//! (`MAC_CFG_FILTER_ACCEPT_GRP`), beacons too while not associated
//! (`MAC_CFG_FILTER_ACCEPT_BEACON`); once associated, `is_assoc` set and the
//! association id the access point gave. HE and EHT stay off (a legacy
//! association), and the NIC does not advertise ACK-enabled aggregation, so
//! `nic_not_ack_enabled` is 1. The coexistence priority bit Linux may set in
//! `data_policy` while the port is closed is left clear: it only raises the
//! join's priority against Bluetooth. With no association this is byte for
//! byte the add the bring-up sends (`cmds::mac_config_add`).

pub const MAC_CONFIG_LEN: usize = 52;

const MAC_TYPE_BSS_STA: u32 = 5;
const FILTER_ACCEPT_GRP: u32 = 1 << 2;
const FILTER_ACCEPT_BEACON: u32 = 1 << 3;

/// `iwl_mac_config_cmd` for MAC 0 at `addr`; `aid` is the association id
/// once associated.
pub fn mac_config(action: u32, addr: [u8; 6], aid: Option<u16>) -> [u8; MAC_CONFIG_LEN] {
    let mut c = [0u8; MAC_CONFIG_LEN];
    c[4..8].copy_from_slice(&action.to_le_bytes());
    c[8..12].copy_from_slice(&MAC_TYPE_BSS_STA.to_le_bytes());
    c[12..18].copy_from_slice(&addr);
    let filter = FILTER_ACCEPT_GRP | if aid.is_none() { FILTER_ACCEPT_BEACON } else { 0 };
    c[20..24].copy_from_slice(&filter.to_le_bytes());
    c[32..36].copy_from_slice(&1u32.to_le_bytes());
    if let Some(aid) = aid {
        c[36] = 1;
        c[40..42].copy_from_slice(&(aid & 0x3FFF).to_le_bytes());
    }
    c
}
