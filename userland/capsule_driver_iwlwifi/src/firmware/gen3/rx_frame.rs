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

//! A received management frame from `REPLY_RX_MPDU_CMD` on an AX210-family
//! part: the 64-byte `iwl_rx_mpdu_desc` (version 3 layout), then the frame,
//! read as Linux v6.12 mvm/rxmq.c `iwl_mvm_rx_mpdu_mq` and
//! `iwl_mvm_create_skb` do: the frame length must fit the packet, a frame
//! with a bad CRC or a FIFO overrun is dropped (Linux only keeps those for
//! monitor mode), the 2-byte alignment pad the device may insert after the
//! header is removed, and the MIC/CRC bytes the device left at the end are cut.
//! Only management frames are taken: the scan wants beacons and probe
//! responses.

use alloc::vec::Vec;

use super::extract_mpdu;

/// `sizeof(struct iwl_rx_mpdu_desc)` on AX210 and newer.
pub const DESC_LEN: usize = 64;
const STATUS_CRC_OK: u32 = 1 << 0;
const STATUS_OVERRUN_OK: u32 = 1 << 1;
const MFLG2_PAD: u8 = 0x20;
const MGMT_HDR: usize = 24;

/// One received management frame and where it was heard.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RxFrame {
    pub frame: Vec<u8>,
    pub channel: u8,
    /// Signal in dBm (negative), or 0 when the device did not measure it.
    pub signal: i8,
}

/// Extract the management frame from an RX MPDU payload, or `None` for a
/// malformed, damaged or non-management one.
pub fn parse(payload: &[u8]) -> Option<RxFrame> {
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
    if raw.len() < MGMT_HDR || raw[0] & 0x0C != 0 {
        return None; // too short, or not a management frame
    }
    let pad = if mac_flags2 & MFLG2_PAD != 0 { 2 } else { 0 };
    let mut len = raw.len().checked_sub(pad)?;
    let mic_crc = (((mac_flags1 & 0xF0) >> 4) as usize) << 1;
    if len > mic_crc {
        len -= mic_crc;
    }
    if len < MGMT_HDR {
        return None;
    }
    let mut frame = Vec::with_capacity(len);
    frame.extend_from_slice(&raw[..MGMT_HDR]);
    frame.extend_from_slice(raw.get(MGMT_HDR + pad..pad + len)?);
    let (ea, eb) = (payload[20 + 20] as i32, payload[20 + 21] as i32);
    let signal = match (ea, eb) {
        (0, 0) => 0,
        (0, e) | (e, 0) => -e,
        (a, b) => -(a.min(b)),
    };
    Some(RxFrame { frame, channel: payload[20 + 22], signal: signal.clamp(-128, 0) as i8 })
}
