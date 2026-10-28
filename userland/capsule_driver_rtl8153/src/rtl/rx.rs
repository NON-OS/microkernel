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

//! The receive framing, as Linux v6.12 r8152.c walks it: each packet a
//! 24-byte descriptor, its first dword's bits 14:0 the packet's length
//! with the 4-byte FCS, then the packet; the next descriptor at the next
//! multiple of 8 from the transfer's start. Nothing here trusts the
//! length: a packet that does not lie wholly inside the bytes that came,
//! or is shorter than any Ethernet frame, ends the walk of its transfer.
//! The descriptor's checksum and VLAN fields are not read.

use nonos_usbnet::xhci::BULK_MAX;

pub const DESC: usize = 24;
pub const FCS: usize = 4;
const ALIGN: usize = 8;
const LEN_BITS: u32 = 0x7fff;
/// The shortest packet a descriptor may give, its FCS counted.
pub const SHORTEST: usize = 60;
/// One bulk IN, the most driver.xhci0 moves in one.
pub const RX_BUFFER: usize = BULK_MAX;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Next {
    /// A frame at `start`, `len` bytes without its FCS; the walk goes on
    /// at `next`.
    Frame { start: usize, len: usize, next: usize },
    /// No descriptor left in the transfer.
    End,
    /// A descriptor whose length is too short or runs past the transfer.
    Broken,
}

/// What lies at `at` in the transfer `got`.
pub fn next(got: &[u8], at: usize) -> Next {
    let rest = got.len().saturating_sub(at);
    if rest <= DESC {
        return Next::End;
    }
    let first = [got[at], got[at + 1], got[at + 2], got[at + 3]];
    let len = (u32::from_le_bytes(first) & LEN_BITS) as usize;
    if len < SHORTEST || len > rest - DESC {
        return Next::Broken;
    }
    let end = at + DESC + len;
    Next::Frame { start: at + DESC, len: len - FCS, next: end.next_multiple_of(ALIGN) }
}
