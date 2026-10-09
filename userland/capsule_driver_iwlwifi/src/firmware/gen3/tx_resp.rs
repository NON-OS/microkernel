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

//! The firmware's answer to a queued frame: `struct iwl_mvm_tx_resp`
//! (fw/api/tx.h, TX_RSP_API_S_VER_6; versions 7 and 8 change only the rate
//! format of `initial_rate`), read as Linux v6.12 mvm/tx.c
//! `iwl_mvm_rx_tx_cmd_single` reads it on the new transmit API:
//! `frame_count` 0, the queue in `tx_queue` at 36, the first status
//! (`agg_tx_status`, 4 bytes each) at 40, and after the `frame_count`
//! statuses the scheduler's next index (`iwl_mvm_get_scd_ssn`: the low 16
//! bits of the word there). Status `TX_STATUS_SUCCESS` (1) or
//! `TX_STATUS_DIRECT_DONE` (2) in the low byte (`TX_STATUS_MSK`) means sent.
//! A response is a packet the firmware wrote, so its frame count is held to
//! its length before any status is read.

const QUEUE_AT: usize = 36;
const STATUS_AT: usize = 40;
const STATUS_SUCCESS: u16 = 0x01;
const STATUS_DIRECT_DONE: u16 = 0x02;

/// What one response says.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TxStatus {
    pub queue: u16,
    /// The first slot the firmware has not finished.
    pub ssn: u16,
    /// The (first) frame was sent.
    pub sent: bool,
}

/// Read a TX response, or `None` for one with no frames or shorter than
/// the statuses and index it claims.
pub fn parse(payload: &[u8]) -> Option<TxStatus> {
    let frames = *payload.first()? as usize;
    if frames == 0 {
        return None;
    }
    let ssn_at = STATUS_AT.checked_add(frames.checked_mul(4)?)?;
    let ssn = payload.get(ssn_at..ssn_at.checked_add(4)?)?;
    let status = u16::from_le_bytes([payload[STATUS_AT], payload[STATUS_AT + 1]]) & 0xFF;
    Some(TxStatus {
        queue: u16::from_le_bytes([payload[QUEUE_AT], payload[QUEUE_AT + 1]]),
        ssn: u16::from_le_bytes([ssn[0], ssn[1]]),
        sent: status == STATUS_SUCCESS || status == STATUS_DIRECT_DONE,
    })
}
