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

//! The attest service's frames for the board: a transport's post, and a
//! reader's question and its answer. The header is the attest service's own
//! (capsule_attest/src/protocol); route_proof_proofs decodes these frames with
//! that service's decoder, so the two cannot drift apart unnoticed.

use crate::board::ANSWER_LEN;
use crate::report::{encode, RouteReport, REPORT_LEN};

/// The service name the board is reached by.
pub const ATTEST_SERVICE: &[u8] = b"attest";
pub const ATTEST_MAGIC: u32 = 0x4154_5354;
pub const ATTEST_VERSION: u16 = 1;
pub const ATTEST_HDR_LEN: usize = 20;
const STATUS_LEN: usize = 4;
pub const OP_ROUTE_REPORT: u16 = 0x0006;
pub const OP_PROOF_ROUTE: u16 = 0x0007;

/// A transport posting `report`.
pub const POST_LEN: usize = ATTEST_HDR_LEN + REPORT_LEN;
/// The reply buffer a reader needs for the answer.
pub const ANSWER_FRAME_LEN: usize = ATTEST_HDR_LEN + STATUS_LEN + ANSWER_LEN;

fn header(out: &mut [u8], op: u16, request_id: u32, payload_len: u32) {
    out[0..4].copy_from_slice(&ATTEST_MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&ATTEST_VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..12].fill(0);
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
}

pub fn post_frame(report: &RouteReport, request_id: u32) -> [u8; POST_LEN] {
    let mut out = [0u8; POST_LEN];
    header(&mut out, OP_ROUTE_REPORT, request_id, REPORT_LEN as u32);
    out[ATTEST_HDR_LEN..].copy_from_slice(&encode(report));
    out
}

pub fn ask_frame(request_id: u32) -> [u8; ATTEST_HDR_LEN] {
    let mut out = [0u8; ATTEST_HDR_LEN];
    header(&mut out, OP_PROOF_ROUTE, request_id, 0);
    out
}

/// The status a reply to `op` / `request_id` carries, and its payload past the
/// status. `None` for a reply to anything else, or one whose lengths disagree.
pub fn reply_body(reply: &[u8], op: u16, request_id: u32) -> Option<(i32, &[u8])> {
    if reply.len() < ATTEST_HDR_LEN + STATUS_LEN {
        return None;
    }
    let u32_at = |i: usize| u32::from_le_bytes([reply[i], reply[i + 1], reply[i + 2], reply[i + 3]]);
    let u16_at = |i: usize| u16::from_le_bytes([reply[i], reply[i + 1]]);
    if u32_at(0) != ATTEST_MAGIC
        || u16_at(4) != ATTEST_VERSION
        || u16_at(6) != op
        || u32_at(12) != request_id
    {
        return None;
    }
    let payload_len = u32_at(16) as usize;
    if payload_len < STATUS_LEN || ATTEST_HDR_LEN.checked_add(payload_len)? > reply.len() {
        return None;
    }
    let status = u32_at(ATTEST_HDR_LEN) as i32;
    Some((status, &reply[ATTEST_HDR_LEN + STATUS_LEN..ATTEST_HDR_LEN + payload_len]))
}
