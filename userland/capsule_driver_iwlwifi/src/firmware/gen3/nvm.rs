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

//! The NVM_GET_INFO reply (`iwl_nvm_get_info_rsp`, version 4 with
//! `IWL_UCODE_TLV_API_REGULATORY_NVM_INFO`, else version 3), read as Linux
//! v6.12 iwl-nvm-parse.c `iwl_get_nvm` does: the reply must be exactly its
//! version's size, the antennas come from the PHY SKU, LAR from the
//! regulatory block, and each channel profile entry's VALID bit says whether
//! the channel at that index (`iwl_uhb_nvm_channels` for a 6 GHz capable
//! part, `iwl_ext_nvm_channels` otherwise) is usable. With LAR the firmware
//! decides the regulatory domain and Linux keeps every channel (they "might
//! become valid later"); the scan here is passive, receive only, so it covers
//! them all then. Only 2.4 and 5 GHz channels are kept: this driver scans
//! those.

use alloc::vec::Vec;

const V4_LEN: usize = 468;
const V3_LEN: usize = 8 + 4 + 8 + 4 + 51 * 2 + 2;
const PROFILE_AT: usize = 20 + 8;
const PROFILE_AT_V3: usize = 20 + 4;
/// `NVM_CHANNEL_VALID`.
const CHANNEL_VALID: u32 = 1 << 0;

/// The 2.4 and 5 GHz part of `iwl_ext_nvm_channels` / `iwl_uhb_nvm_channels`
/// (identical up to there; the 6-7 GHz tail is not scanned).
pub const NVM_CHANNELS: [u8; 51] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 40, 44, 48, 52, 56, 60, 64, 68, 72, 76, 80, 84, 88,
    92, 96, 100, 104, 108, 112, 116, 120, 124, 128, 132, 136, 140, 144, 149, 153, 157, 161, 165, 169,
    173, 177, 181,
];

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NvmInfo {
    pub valid_tx_ant: u8,
    pub valid_rx_ant: u8,
    pub lar_enabled: bool,
    /// Usable 2.4 and 5 GHz channel numbers, in NVM order.
    pub channels: Vec<u8>,
}

/// Parse the reply; `v4` is whether the firmware sets the API flag.
pub fn parse(payload: &[u8], v4: bool) -> Option<NvmInfo> {
    let (len, profile_at, width) = if v4 { (V4_LEN, PROFILE_AT, 4) } else { (V3_LEN, PROFILE_AT_V3, 2) };
    if payload.len() != len {
        return None;
    }
    let tx = le32(payload, 12)? as u8;
    let rx = le32(payload, 16)? as u8;
    let lar = le32(payload, 20)? != 0;
    let mut channels = Vec::new();
    for (i, &ch) in NVM_CHANNELS.iter().enumerate() {
        let at = profile_at + i * width;
        let flags = if width == 4 {
            le32(payload, at)?
        } else {
            u16::from_le_bytes(payload.get(at..at + 2)?.try_into().ok()?) as u32
        };
        if lar || flags & CHANNEL_VALID != 0 {
            channels.push(ch);
        }
    }
    Some(NvmInfo { valid_tx_ant: tx, valid_rx_ant: rx, lar_enabled: lar, channels })
}

fn le32(d: &[u8], o: usize) -> Option<u32> {
    let b = d.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
