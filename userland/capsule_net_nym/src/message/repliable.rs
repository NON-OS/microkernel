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

use alloc::vec::Vec;

use super::types::{
    SENDER_TAG_SIZE, SURB_BASE_BYTES, SURB_KEY_ROTATION_UNKNOWN, TAG_ADDITIONAL_SURBS, TAG_DATA,
    TYPE_REPLIABLE,
};
use crate::sphinx::constants::PAYLOAD_KEY_SEED_SIZE;

/// Build a repliable data message.
///
/// This is the shape a network requester expects from a client that wants an
/// answer. The sender tag is what the far end quotes to reach us again, and
/// the reply surbs are the only route it has back: it never learns where we
/// are, so without one attached it has no way to answer at all.
///
/// Layout is the message type, the sender tag, the content tag, the reply
/// blocks as `blocks_header` lays them out, then the request.
pub fn repliable_data(
    sender_tag: &[u8; SENDER_TAG_SIZE],
    reply_surbs: &[Vec<u8>],
    message: &[u8],
) -> Vec<u8> {
    let mut out = with_blocks(sender_tag, TAG_DATA, reply_surbs, message.len());
    out.extend_from_slice(message);
    out
}

/// Build a message carrying nothing but reply blocks.
///
/// A recipient keeps a reserve it will not spend and stops answering once it
/// is down to it, saying so rather than going quiet. This is the answer to
/// that: no request, no data, just more ways home.
pub fn repliable_additional_surbs(
    sender_tag: &[u8; SENDER_TAG_SIZE],
    reply_surbs: &[Vec<u8>],
) -> Vec<u8> {
    with_blocks(sender_tag, TAG_ADDITIONAL_SURBS, reply_surbs, 0)
}

/// The head of a repliable message and its reply blocks, with room for `more`.
///
/// The blocks go as a big endian u16 count, the hops each block has, the key
/// rotation they were built against, then the blocks back to back. Every block
/// is built alike, so the first one's width says the hops of all of them.
fn with_blocks(
    sender_tag: &[u8; SENDER_TAG_SIZE],
    tag: u8,
    reply_surbs: &[Vec<u8>],
    more: usize,
) -> Vec<u8> {
    let surb_bytes: usize = reply_surbs.iter().map(|s| s.len()).sum();
    let mut out = Vec::with_capacity(1 + SENDER_TAG_SIZE + 1 + 4 + surb_bytes + more);
    out.push(TYPE_REPLIABLE);
    out.extend_from_slice(sender_tag);
    out.push(tag);
    let count = reply_surbs.len().min(u16::MAX as usize);
    out.extend_from_slice(&(count as u16).to_be_bytes());
    out.push(reply_surbs.first().map_or(0, |s| block_hops(s.len())));
    out.push(SURB_KEY_ROTATION_UNKNOWN);
    for surb in &reply_surbs[..count] {
        out.extend_from_slice(surb);
    }
    out
}

/// The hops of a reply block `len` bytes wide: one seed for each.
pub fn block_hops(len: usize) -> u8 {
    (len.saturating_sub(SURB_BASE_BYTES) / PAYLOAD_KEY_SEED_SIZE) as u8
}

/// Pad a message out to whole packets before it is split.
///
/// The marker byte is what tells the far end where the message stopped and
/// the padding began, so it is written even when the message already lands on
/// the boundary. Every packet then carries the same number of bytes whatever
/// it holds, which is the point: a length that varied with the request would
/// describe it to anyone counting.
pub fn pad_to_packets(message: Vec<u8>, plaintext_per_packet: usize) -> Option<Vec<u8>> {
    if plaintext_per_packet == 0 {
        return None;
    }
    let mut out = message;
    out.push(1u8);
    let used = out.len().div_ceil(plaintext_per_packet);
    out.resize(used * plaintext_per_packet, 0u8);
    Some(out)
}

/// Strip the padding a message was sent with.
pub fn unpad(padded: &[u8]) -> Option<&[u8]> {
    let marker = padded.iter().rposition(|&b| b == 1)?;
    // Everything after the marker has to be padding, or this is not the
    // message that was sent.
    if padded[marker + 1..].iter().any(|&b| b != 0) {
        return None;
    }
    Some(&padded[..marker])
}
