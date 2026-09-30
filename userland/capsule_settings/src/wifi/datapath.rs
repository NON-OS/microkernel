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

//! Read the driver's data-path frame counts from the status op. These split a
//! stuck DHCP: no TX means net_core handed nothing down, RX frames without parsed
//! frames means the AP replies but the frames never decrypt.

use nonos_wifi_client::{find, OP_STATUS, WIFI_HDR};

const STATUS_TIMEOUT_MS: u64 = 500;

/// The driver's TX and RX frame counts since bring-up, plus the number of
/// net_core link-protocol requests it has answered (zero means the stack never
/// reached the radio).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct DataPath {
    pub tx_ok: u32,
    pub tx_drop: u32,
    pub rx_ring: u32,
    pub rx_eth: u32,
    pub netif_reqs: u32,
    pub rx_err: u32,
    /// The efuse control, address and LDO registers as the last stalled read left
    /// them. All zero means no read has stalled since boot, so a radio that
    /// reports the efuse stage with zeros here never entered the polling loop.
    pub efuse_ctl: u32,
    pub efuse_addr: u32,
    pub efuse_ldo: u32,
    /// The BAR the register window was taken from and the low half of its
    /// address. rtw88 uses bar_id 2 on this chip, so anything else means the
    /// registers are being read from the wrong window.
    pub bar_index: u32,
    pub window_va: u32,
}

/// Query the driver for its data-path counts. `None` when no Wi-Fi driver is
/// registered or it does not answer with the counters (the iwlwifi driver
/// answers the status op with its stage alone).
pub fn driver_datapath() -> Option<DataPath> {
    let driver = find()?;
    let mut resp = [0u8; WIFI_HDR + 45];
    let n = driver.request(OP_STATUS, &[], &mut resp, STATUS_TIMEOUT_MS)? as i64;
    if n < (WIFI_HDR + 25) as i64 {
        return None;
    }
    let b = &resp[WIFI_HDR + 1..];
    // The efuse words are newer than the counters, so a driver built before them
    // answers the shorter reply and those three stay zero rather than failing the
    // whole read.
    let long = n >= (WIFI_HDR + 45) as i64;
    let word = |i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
    Some(DataPath {
        tx_ok: word(0),
        tx_drop: word(4),
        rx_ring: word(8),
        rx_eth: word(12),
        netif_reqs: word(16),
        rx_err: word(20),
        efuse_ctl: if long { word(24) } else { 0 },
        efuse_addr: if long { word(28) } else { 0 },
        efuse_ldo: if long { word(32) } else { 0 },
        bar_index: if long { word(36) } else { 0xFFFF_FFFF },
        window_va: if long { word(40) } else { 0 },
    })
}
