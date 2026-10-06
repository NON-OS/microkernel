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

//! A received frame of any kind from `REPLY_RX_MPDU_CMD` while joining or
//! joined: management frames for the join, data frames for the handshake
//! and the link. Read as Linux v6.12 mvm/rxmq.c `iwl_mvm_rx_mpdu_mq`,
//! `iwl_mvm_rx_crypto` and `iwl_mvm_create_skb` read them, from the same
//! 64-byte descriptor the scan reads (`rx_frame`):
//!
//! - a bad CRC or a FIFO overrun drops the frame (Linux keeps those only for
//!   monitor mode);
//! - a protected frame the firmware decrypted with CCM (`status` bits 8-10
//!   `IWL_RX_MPDU_STATUS_SEC_CCM`) counts only with `MIC_OK`, and keeps its
//!   CCMP header (8 bytes); one the firmware had no key for
//!   (`SEC_NONE`, `SEC_UNKNOWN`) is passed on still encrypted, for the
//!   station to decrypt; any other cipher is dropped;
//! - the 2-byte alignment pad the device may insert (`IWL_RX_MPDU_MFLG2_PAD`)
//!   sits after the header and, for a decrypted frame, after its CCMP header,
//!   and is removed;
//! - the MIC and CRC bytes the receive accelerator left (`mac_flags1` bits
//!   4-7, in 2-byte units) are cut. A frame the firmware decrypted therefore
//!   arrives without its MIC (`RX_FLAG_MIC_STRIPPED`).
//!
//! Control frames are dropped. Every length comes from the device and is
//! checked before it is used.

use alloc::vec::Vec;

use nonos_wifi_core::dot11::ccmp::view;

use super::extract_mpdu;
use super::rx_frame::DESC_LEN;

const STATUS_CRC_OK: u32 = 1 << 0;
const STATUS_OVERRUN_OK: u32 = 1 << 1;
const STATUS_MIC_OK: u32 = 1 << 6;
const STATUS_SEC_SHIFT: u32 = 8;
const SEC_NONE: u32 = 0;
const SEC_CCM: u32 = 2;
const SEC_UNKNOWN: u32 = 7;
const MFLG2_PAD: u8 = 0x20;
const FC_PROTECTED: u16 = 0x4000;
const TYPE_CONTROL: u16 = 1;
/// `IEEE80211_CCMP_HDR_LEN`.
const CCMP_HDR: usize = 8;

/// One received frame, and whether the firmware decrypted it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RxMpdu {
    pub frame: Vec<u8>,
    /// A protected frame the firmware decrypted and whose MIC it checked;
    /// the CCMP header is in place, the MIC is not.
    pub decrypted: bool,
}

/// Extract the frame from an RX MPDU payload, or `None` for a malformed,
/// damaged, undecryptable or control frame.
pub fn parse(payload: &[u8]) -> Option<RxMpdu> {
    if payload.len() < DESC_LEN {
        return None;
    }
    let mac_flags1 = payload[2];
    let mac_flags2 = payload[3];
    let status = u32::from_le_bytes([payload[12], payload[13], payload[14], payload[15]]);
    if status & (STATUS_CRC_OK | STATUS_OVERRUN_OK) != (STATUS_CRC_OK | STATUS_OVERRUN_OK) {
        return None;
    }
    let raw = extract_mpdu(payload, DESC_LEN)?;
    let hdr_len = view(raw)?.hdr_len;
    let fc = u16::from_le_bytes([raw[0], raw[1]]);
    if (fc >> 2) & 0x3 == TYPE_CONTROL {
        return None;
    }
    let decrypted = if fc & FC_PROTECTED == 0 {
        false
    } else {
        match (status >> STATUS_SEC_SHIFT) & 0x7 {
            SEC_CCM if status & STATUS_MIC_OK != 0 => true,
            SEC_NONE | SEC_UNKNOWN => false,
            _ => return None,
        }
    };
    let keep = hdr_len.checked_add(if decrypted { CCMP_HDR } else { 0 })?;
    let pad = if mac_flags2 & MFLG2_PAD != 0 { 2 } else { 0 };
    let mut len = raw.len().checked_sub(pad)?;
    let mic_crc = (((mac_flags1 & 0xF0) >> 4) as usize) << 1;
    if len > mic_crc {
        len -= mic_crc;
    }
    if len < keep {
        return None;
    }
    let mut frame = Vec::with_capacity(len);
    frame.extend_from_slice(raw.get(..keep)?);
    frame.extend_from_slice(raw.get(keep + pad..pad + len)?);
    Some(RxMpdu { frame, decrypted })
}
