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

//! The access point as a peer in the firmware's station table.
//!
//! `struct iwl_sta_cfg_cmd_v1` (fw/api/mac-cfg.h), STA_CMD_API_S_VER_1, 96
//! bytes, the layout Linux sends when the firmware does not list
//! STA_CONFIG_CMD (`iwl_mvm_mld_send_sta_cmd`): `sta_id` 0, `link_id` 4,
//! `peer_mld_address` 8, `peer_link_address` 16, `station_type` 24,
//! `assoc_id` 28, `beamform_flags` 32, `mfp` 36, `mimo` 40,
//! `mimo_protection` 44, `ack_enabled` 48, `trig_rnd_alloc` 52,
//! `tx_ampdu_spacing` 56, `tx_ampdu_max_size` 60, `sp_length` 64,
//! `uapsd_acs` 68, `pkt_ext` (`iwl_he_pkt_ext_v2`, 20 bytes) 72, `htc_flags`
//! 92.
//!
//! Filled as Linux v6.12 mvm/mld-sta.c `iwl_mvm_mld_cfg_sta` fills it for the
//! access point of a legacy, non-QoS association: station 0 on link 0, the
//! BSSID as both addresses, `STATION_TYPE_PEER` 0, the association id once
//! associated, one spatial stream (`mimo` 0) with SMPS off, no aggregation
//! (no HT capabilities, so spacing and size 0) and no U-APSD (no WMM).
//! `mfp` follows `IWL_UCODE_TLV_CAPA_STA_EXP_MFP_SUPPORT`, which this
//! firmware has: 1 until the station is authorized, then whether management
//! frame protection was negotiated.
//!
//! `struct iwl_remove_sta_cmd`, 4 bytes: `sta_id`.

pub const STA_CONFIG_V1_LEN: usize = 96;
pub const STA_REMOVE_LEN: usize = 4;
/// The access point's station id.
pub const AP_STA_ID: u32 = 0;
/// `STATION_TYPE_PEER`.
const STATION_TYPE_PEER: u32 = 0;

/// The access point's entry: its address, the association id once
/// associated, and the `mfp` word.
pub fn sta_config(link_id: u32, bssid: [u8; 6], aid: u16, mfp: bool) -> [u8; STA_CONFIG_V1_LEN] {
    let mut c = [0u8; STA_CONFIG_V1_LEN];
    c[0..4].copy_from_slice(&AP_STA_ID.to_le_bytes());
    c[4..8].copy_from_slice(&link_id.to_le_bytes());
    c[8..14].copy_from_slice(&bssid);
    c[16..22].copy_from_slice(&bssid);
    c[24..28].copy_from_slice(&STATION_TYPE_PEER.to_le_bytes());
    c[28..32].copy_from_slice(&u32::from(aid).to_le_bytes());
    c[36..40].copy_from_slice(&u32::from(mfp).to_le_bytes());
    c
}

/// `iwl_remove_sta_cmd` for the access point.
pub fn sta_remove() -> [u8; STA_REMOVE_LEN] {
    AP_STA_ID.to_le_bytes()
}

/// The station mask commands that name stations by bit use.
pub const AP_STA_MASK: u32 = 1 << AP_STA_ID;
