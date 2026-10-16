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

//! The scan request, built for a list of channels.

use alloc::vec::Vec;

use super::consts::*;

/// Build the request with `uid` for `channels` (2.4 and 5 GHz numbers).
/// `None` for no channels or more than the request holds.
pub fn request(channels: &[u8], uid: u32) -> Option<Vec<u8>> {
    if channels.is_empty() || channels.len() > MAX_CHANNELS {
        return None;
    }
    let mut c = alloc::vec![0u8; SCAN_REQ_V17_LEN];
    c[0..4].copy_from_slice(&uid.to_le_bytes());
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
