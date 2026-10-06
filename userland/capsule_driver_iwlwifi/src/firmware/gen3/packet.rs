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

//! One packet the firmware posted in a receive buffer: `struct iwl_rx_packet`,
//! a length-and-flags word, the command header (`cmd`, `group_id`,
//! `sequence`) and the payload. AX210-family parts put one packet per buffer
//! (`iwl_pcie_rx_handle_rb` stops after the first). The device's length is
//! checked against the buffer: a packet claiming more than the buffer holds,
//! less than its own header, or the invalid-frame marker, is refused.

/// `FH_RSCSR_FRAME_SIZE_MSK`.
const FRAME_SIZE_MASK: u32 = 0x0000_3FFF;
/// `FH_RSCSR_FRAME_INVALID`: the end-of-buffer marker.
const FRAME_INVALID: u32 = 0x5555_0000;
/// The length word plus the 4-byte command header.
pub const PACKET_HDR: usize = 8;
/// `SEQ_RX_FRAME`: the firmware originated this packet (not a reply).
pub const SEQ_RX_FRAME: u16 = 0x8000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Packet<'a> {
    pub cmd: u8,
    pub group: u8,
    pub sequence: u16,
    pub payload: &'a [u8],
}

impl Packet<'_> {
    /// A reply to a host command (as opposed to a notification).
    pub fn is_reply(&self) -> bool {
        self.sequence & SEQ_RX_FRAME == 0
    }
}

/// Parse the packet at the start of receive buffer `rb`.
pub fn parse(rb: &[u8]) -> Option<Packet<'_>> {
    let word = u32::from_le_bytes(rb.get(0..4)?.try_into().ok()?);
    if word == FRAME_INVALID {
        return None;
    }
    // The length counts the command header and payload, not the word itself.
    let len = (word & FRAME_SIZE_MASK) as usize;
    let total = len.checked_add(4)?;
    if total < PACKET_HDR || total > rb.len() {
        return None;
    }
    Some(Packet {
        cmd: rb[4],
        group: rb[5],
        sequence: u16::from_le_bytes([rb[6], rb[7]]),
        payload: &rb[PACKET_HDR..total],
    })
}
