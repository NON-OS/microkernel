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

//! The driver's status op asked, within half a second, and its counters and
//! firmware report read out of the answer (`DataPath`).

use super::datapath::DataPath;
use nonos_wifi_client::{find, OP_STATUS, WIFI_HDR};

const STATUS_TIMEOUT_MS: u64 = 500;

/// The firmware report's offset past the stage byte (it follows the 52 bytes of
/// counters and diagnosis), and where its 25 bytes end, counted from the stage.
const FW_REPORT_AT: usize = 52;

const FW_REPORT_END: usize = 1 + FW_REPORT_AT + 25;

/// Query the driver for its data-path counts. `None` when no Wi-Fi driver is
/// registered or it does not answer with the counters (the iwlwifi driver
/// answers the status op with its stage alone).
pub fn driver_datapath() -> Option<DataPath> {
    let driver = find()?;
    let mut resp = [0u8; WIFI_HDR + FW_REPORT_END];
    let n = driver.request(OP_STATUS, &[], &mut resp, STATUS_TIMEOUT_MS)? as i64;
    if n < (WIFI_HDR + 25) as i64 {
        return None;
    }
    let b = &resp[WIFI_HDR + 1..];
    // The efuse words are newer than the counters, so a driver built before them
    // answers the shorter reply and those three stay zero rather than failing the
    // whole read.
    let long = n >= (WIFI_HDR + 45) as i64;
    let fw = n >= (WIFI_HDR + FW_REPORT_END) as i64;
    let word = |i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
    Some(DataPath {
        tx_ok: word(0),
        tx_drop: word(4),
        rx_ring: word(8),
        rx_eth: word(12),
        netif_reqs: word(16),
        rx_err: word(20),
        efuse_flags: if long { word(24) } else { 0 },
        efuse_stalled_at: if long { word(28) } else { 0 },
        efuse_bank_ldo: if long { word(32) } else { 0 },
        fw_step: if fw { b[FW_REPORT_AT] } else { 0 },
        fw_chunks: if fw { word(FW_REPORT_AT + 1) } else { 0 },
        fw_ctrl: if fw { word(FW_REPORT_AT + 5) } else { 0 },
    })
}
