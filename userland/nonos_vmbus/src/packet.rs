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

//! A packet on a ring (QEMU's vmbus_packet_hdr): a 16-byte header, a
//! descriptor, the data, padding to 8 bytes, then a trailer the reader
//! skips.
//!
//! ```text
//!   0 type u16   2 offset_qwords u16   4 len_qwords u16   6 flags u16
//!   8 transaction_id u64
//! ```
//!
//! `offset_qwords` is where the data starts and `len_qwords` where the
//! packet ends, both in 8-byte words from the header's start.

pub const HEADER_BYTES: usize = 16;
pub const TRAILER_BYTES: usize = 8;

pub const PACKET_DATA_INBAND: u16 = 0x6;
pub const PACKET_DATA_USING_XFER_PAGES: u16 = 0x7;
pub const PACKET_DATA_USING_GPA_DIRECT: u16 = 0x9;
pub const PACKET_COMP: u16 = 0xb;

/// The sender wants a completion packet back.
pub const FLAG_REQUEST_COMPLETION: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub ty: u16,
    pub flags: u16,
    pub transaction_id: u64,
    /// Where the data starts and the packet ends, in bytes; filled in by
    /// `sized` when sending, read from the ring when receiving.
    pub data_at: usize,
    pub end: usize,
}

impl Header {
    pub const fn new(ty: u16, flags: u16, transaction_id: u64) -> Self {
        Self {
            ty,
            flags,
            transaction_id,
            data_at: 0,
            end: 0,
        }
    }

    /// The header's bytes for a descriptor of `desc` bytes (a multiple of
    /// 8) and `data` bytes, and the packet's whole size on the ring with
    /// its trailer. None when it cannot be encoded.
    pub fn sized(&self, desc: usize, data: usize) -> Option<([u8; HEADER_BYTES], usize)> {
        if !desc.is_multiple_of(8) {
            return None;
        }
        let offset = (HEADER_BYTES + desc) / 8;
        let len = offset + data.div_ceil(8);
        let (offset16, len16) = (u16::try_from(offset).ok()?, u16::try_from(len).ok()?);
        let mut b = [0u8; HEADER_BYTES];
        b[0..2].copy_from_slice(&self.ty.to_le_bytes());
        b[2..4].copy_from_slice(&offset16.to_le_bytes());
        b[4..6].copy_from_slice(&len16.to_le_bytes());
        b[6..8].copy_from_slice(&self.flags.to_le_bytes());
        b[8..16].copy_from_slice(&self.transaction_id.to_le_bytes());
        Some((b, len * 8 + TRAILER_BYTES))
    }

    /// A header the host wrote, held to its own lengths: the data starts
    /// after the header and no later than the packet ends.
    pub fn parse(b: &[u8; HEADER_BYTES]) -> Option<Self> {
        let u16_at = |i: usize| u16::from_le_bytes([b[i], b[i + 1]]);
        let (offset, len) = (u16_at(2) as usize, u16_at(4) as usize);
        if offset < HEADER_BYTES / 8 || offset > len {
            return None;
        }
        Some(Self {
            ty: u16_at(0),
            flags: u16_at(6),
            transaction_id: u64::from_le_bytes(b[8..16].try_into().ok()?),
            data_at: offset * 8,
            end: len * 8,
        })
    }

    /// The bytes the packet occupies before its trailer.
    pub fn bytes(&self) -> usize {
        self.end
    }
}

/// A GPA-direct descriptor (QEMU's vmbus_pkt_gpa_direct) of one range: the
/// buffer at byte `offset` in the first of `pfns`, `len` bytes long. Writes
/// into `out` and returns its length, or None when `out` is too small.
pub fn gpa_direct(out: &mut [u8], len: u32, offset: u32, pfns: &[u64]) -> Option<usize> {
    let n = 16 + 8 * pfns.len();
    let out = out.get_mut(..n)?;
    out[0..4].copy_from_slice(&0u32.to_le_bytes());
    out[4..8].copy_from_slice(&1u32.to_le_bytes());
    out[8..12].copy_from_slice(&len.to_le_bytes());
    out[12..16].copy_from_slice(&offset.to_le_bytes());
    for (i, p) in pfns.iter().enumerate() {
        out[16 + 8 * i..24 + 8 * i].copy_from_slice(&p.to_le_bytes());
    }
    Some(n)
}

/// A transfer-pages descriptor (QEMU's vmbus_pkt_xferpg, OpenVMM's
/// TransferPageHeader): the transfer page set, then `range_count` ranges of
/// (byte_count, byte_offset) into it. The set id and the ranges, or None
/// when the descriptor is shorter than its count says.
pub fn xfer_pages(desc: &[u8]) -> Option<(u16, XferRanges<'_>)> {
    let head = desc.get(..8)?;
    let set = u16::from_le_bytes([head[0], head[1]]);
    let count = u32::from_le_bytes([head[4], head[5], head[6], head[7]]) as usize;
    let ranges = desc.get(8..8 + count.checked_mul(8)?)?;
    Some((set, XferRanges(ranges)))
}

/// The ranges of a transfer-pages descriptor, as (byte_count, byte_offset).
#[derive(Clone, Copy, Debug)]
pub struct XferRanges<'a>(&'a [u8]);

impl Iterator for XferRanges<'_> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        let (r, rest) = self.0.split_at_checked(8)?;
        self.0 = rest;
        let count = u32::from_le_bytes([r[0], r[1], r[2], r[3]]);
        let offset = u32::from_le_bytes([r[4], r[5], r[6], r[7]]);
        Some((count, offset))
    }
}
