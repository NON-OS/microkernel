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

//! The frames in one bulk IN transfer. A device may batch several
//! PACKET_MSGs into it (Remote NDIS 1.0, 2.2.13; Linux rndis_rx_fixup),
//! and every length in them comes from the device, so each is checked
//! against the message and the message against the transfer.

use alloc::collections::VecDeque;
use core::ops::Range;

use nonos_usbnet::nic::{ETH_FRAME_MAX, ETH_HEADER};

use super::message::{le32, MSG_PACKET};
use super::packet::{DATA_OFFSET, PACKET_HDR};

/// Queue the frames of transfer `t` as ranges of it, in order; the number
/// of messages dropped. A message of another type, or one whose
/// MessageLength runs short of a header or past the transfer, ends the
/// walk: what follows cannot be found (Linux drops the rest the same
/// way). Zero padding after the last message ends it quietly.
pub fn frames(t: &[u8], queue: &mut VecDeque<Range<usize>>) -> u32 {
    let (mut at, mut dropped) = (0usize, 0u32);
    while let (Some(kind), Some(len)) = (le32(t, at), le32(t, at + 4)) {
        let len = len as usize;
        if kind != MSG_PACKET || len < PACKET_HDR || len > t.len() - at {
            dropped += u32::from(kind != 0 || len != 0);
            break;
        }
        match data(&t[at..at + len]) {
            Some(r) => queue.push_back(at + r.start..at + r.end),
            None => dropped += 1,
        }
        at += len;
    }
    dropped
}

/// The frame inside one message `m`: its data must lie after the header
/// and inside MessageLength, and be an Ethernet frame without FCS.
fn data(m: &[u8]) -> Option<Range<usize>> {
    let off = le32(m, 8)?;
    let len = le32(m, 12)? as usize;
    if off < DATA_OFFSET || !(ETH_HEADER..=ETH_FRAME_MAX).contains(&len) {
        return None;
    }
    let start = (off as usize).checked_add(8)?;
    let end = start.checked_add(len)?;
    (end <= m.len()).then_some(start..end)
}
