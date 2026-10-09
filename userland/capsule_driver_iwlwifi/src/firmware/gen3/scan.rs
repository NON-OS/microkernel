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

//! The UMAC scan request, version 17 (`iwl_scan_req_umac_v17`, 1940 bytes),
//! filled as Linux v6.12 mvm/scan.c `iwl_mvm_scan_umac_v14_and_above` fills
//! it for a regular scan with no SSIDs while unassociated: forced passive (no
//! probe request is ever sent, so no address and no network name goes on the
//! air), pass all results, adaptive dwell, one iteration, the unassociated
//! timing (no time off channel limits), link 0 as the scan's reference, each
//! channel with its band in the top flag bits. EBS (energy-based scan) is off,
//! which Linux also does after an EBS failure. The probe parameters stay zero.
//! The completion (`iwl_umac_scan_complete`) is read back by uid.

use alloc::vec::Vec;

/// The request's size.
pub const SCAN_REQ_V17_LEN: usize = 1940;
/// `SCAN_MAX_NUM_CHANS_V3`.
pub const MAX_CHANNELS: usize = 67;
/// The uid of the one scan this driver runs at a time.
pub const SCAN_UID: u32 = 0;

const GEN_FLAGS_PASS_ALL: u16 = 1 << 1;
const GEN_FLAGS_ADAPTIVE_DWELL: u16 = 1 << 7;
const GEN_FLAGS_FORCE_PASSIVE: u16 = 1 << 11;
const SCAN_PRIORITY_EXT_6: u32 = 6;
const DWELL_ACTIVE: u8 = 10;
const DWELL_PASSIVE: u8 = 110;
const ADWELL_LB_N_APS: u8 = 2;
const ADWELL_HB_N_APS: u8 = 8;
const ADWELL_N_APS_SOCIAL: u8 = 10;
const ADWELL_MAX_BUDGET_FULL: u16 = 300;
const CHANNEL_FLAG_ENABLE_CHAN_ORDER: u8 = 1 << 5;
const N_APS_GO_FRIENDLY: u8 = 10;
const N_APS_SOCIAL_CHS: u8 = 2;
const BAND_POS: u32 = 30;
const PHY_BAND_5: u32 = 0;
const PHY_BAND_24: u32 = 1;

// Offsets inside the request.
const GENERAL: usize = 8;
const CHANNELS: usize = GENERAL + 36;
const CHANNEL_CFG: usize = CHANNELS + 4;
const PERIODIC: usize = CHANNELS + 540;

/// Build the request for `channels` (2.4 and 5 GHz numbers). `None` for no
/// channels or more than the request holds.
pub fn request(channels: &[u8]) -> Option<Vec<u8>> {
    if channels.is_empty() || channels.len() > MAX_CHANNELS {
        return None;
    }
    let mut c = alloc::vec![0u8; SCAN_REQ_V17_LEN];
    c[0..4].copy_from_slice(&SCAN_UID.to_le_bytes());
    c[4..8].copy_from_slice(&SCAN_PRIORITY_EXT_6.to_le_bytes());

    let g = GENERAL;
    let flags = GEN_FLAGS_FORCE_PASSIVE | GEN_FLAGS_PASS_ALL | GEN_FLAGS_ADAPTIVE_DWELL;
    c[g..g + 2].copy_from_slice(&flags.to_le_bytes());
    c[g + 3] = 0; // scan_start_mac_or_link_id: link 0
    c[g + 4] = DWELL_ACTIVE;
    c[g + 5] = DWELL_ACTIVE;
    c[g + 6] = ADWELL_LB_N_APS;
    c[g + 7] = ADWELL_HB_N_APS;
    c[g + 8] = ADWELL_N_APS_SOCIAL;
    c[g + 10..g + 12].copy_from_slice(&ADWELL_MAX_BUDGET_FULL.to_le_bytes());
    // max_out_of_time and suspend_time stay 0 (IWL_SCAN_TYPE_UNASSOC).
    c[g + 28..g + 32].copy_from_slice(&SCAN_PRIORITY_EXT_6.to_le_bytes());
    c[g + 32] = DWELL_PASSIVE;
    c[g + 33] = DWELL_PASSIVE;

    c[CHANNELS] = CHANNEL_FLAG_ENABLE_CHAN_ORDER;
    c[CHANNELS + 1] = channels.len() as u8;
    c[CHANNELS + 2] = N_APS_GO_FRIENDLY;
    c[CHANNELS + 3] = N_APS_SOCIAL_CHS;
    for (i, &ch) in channels.iter().enumerate() {
        let at = CHANNEL_CFG + i * 8;
        let band = if ch <= 14 { PHY_BAND_24 } else { PHY_BAND_5 };
        c[at..at + 4].copy_from_slice(&(band << BAND_POS).to_le_bytes());
        c[at + 4] = ch;
        // Linux writes `v2.iter_count`, the same byte as `v5.iter_count`.
        c[at + 6] = 1;
    }

    // One plan: one iteration, no interval; no start delay.
    c[PERIODIC + 2] = 1;
    Some(c)
}

/// A scan completion: its uid and status (`IWL_SCAN_OFFLOAD_COMPLETED` is 1,
/// `_ABORTED` 2). `None` for a payload shorter than the structure.
pub fn complete(payload: &[u8]) -> Option<(u32, u8)> {
    if payload.len() < 16 {
        return None;
    }
    let uid = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    Some((uid, payload[6]))
}
