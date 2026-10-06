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

//! Checking a received NTH16 and each NDP16 before anything they point at
//! is read, as Linux cdc_ncm_rx_verify_nth16 and cdc_ncm_rx_verify_ndp16
//! do. The bytes come from the device; every read is bounded by the
//! transfer.

use super::ntb_out::{NDP16_ONE, NTH16_LEN, NTH16_SIGN};

/// dwSignature, wLength and wNextNdpIndex: what precedes the entries.
pub const NDP16_HEADER: usize = 8;
const ENTRY: usize = 4;
/// USB_CDC_NCM_NDP16_INDEX_MIN (NCM 1.0, 3.2.1): an NDP cannot start
/// inside the NTH16.
const NDP16_INDEX_MIN: usize = 0x0C;

pub fn le16(raw: &[u8], at: usize) -> usize {
    u16::from_le_bytes([raw[at], raw[at + 1]]) as usize
}

pub fn le32(raw: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([raw[at], raw[at + 1], raw[at + 2], raw[at + 3]])
}

/// The first NDP's index, when `raw` holds an NTH16 and an NDP16 header's
/// worth of bytes, has the signature, and a wBlockLength within `rx_max`.
pub fn verify_nth16(raw: &[u8], rx_max: usize) -> Option<usize> {
    if raw.len() < NTH16_LEN + NDP16_HEADER || le32(raw, 0) != NTH16_SIGN {
        return None;
    }
    (le16(raw, 8) <= rx_max).then(|| le16(raw, 10))
}

/// The datagram entries before the terminating one in the NDP16 at `at`.
/// Linux checks the entries' end from the start of the transfer; here it is
/// checked from the NDP, so no entry is read past the transfer.
pub fn verify_ndp16(raw: &[u8], at: usize) -> Option<usize> {
    if at < NDP16_INDEX_MIN || at.checked_add(NDP16_HEADER)? > raw.len() {
        return None;
    }
    let len = le16(raw, at + 4);
    if len < NDP16_ONE {
        return None;
    }
    let n = (len - NDP16_HEADER) / ENTRY - 1;
    (at + NDP16_HEADER + n * ENTRY <= raw.len()).then_some(n)
}
