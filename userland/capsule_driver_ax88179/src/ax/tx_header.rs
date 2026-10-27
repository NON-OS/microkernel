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

//! The chip's send framing (as Linux v6.12 builds it, read for behaviour):
//! an 8-byte header before each frame, the frame's length as a 32-bit word,
//! then a second word holding the segment size for offloads (0: none here)
//! and, in bits 31 and 15 together, the mark that one padding byte follows.

/// The header's length.
pub const TX_HEADER: usize = 8;
const PADDED: u32 = 0x8000_8000;

/// The bulk OUT length for a frame of `len` bytes behind its header, and
/// whether a padding byte ends it. A transfer that fills its last packet
/// would need a zero-length packet after it, which the bus has no way to
/// ask for; one byte more ends it with a short packet, and the header says
/// the byte is not the frame's. A multiple of 64 is padded too: the
/// controller driver moves a high-speed or SuperSpeed pipe in 512 or
/// 1024-byte packets whatever the descriptor says.
pub fn framed(len: usize, max_packet: u16) -> (usize, bool) {
    let (total, mps) = (TX_HEADER + len, max_packet as usize);
    let pad = total.is_multiple_of(64) || (mps != 0 && total.is_multiple_of(mps));
    (total + pad as usize, pad)
}

/// The header for a frame of `len` bytes, at most a full frame.
pub fn header(len: usize, padded: bool) -> [u8; TX_HEADER] {
    let second = if padded { PADDED } else { 0 };
    let mut h = [0u8; TX_HEADER];
    h[..4].copy_from_slice(&(len as u32).to_le_bytes());
    h[4..].copy_from_slice(&second.to_le_bytes());
    h
}
