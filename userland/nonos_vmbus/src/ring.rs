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

//! A channel's two rings (QEMU's vmbus_ring_buffer, Linux's
//! hv_ring_buffer). Each is a header page, then data pages that wrap:
//!
//! ```text
//!   header  0 write_index  4 read_index  8 interrupt_mask  12 pending_send_sz
//!           64 feature_bits
//!   data    packets, each 8-byte aligned, then an 8-byte trailer
//! ```
//!
//! The indices are byte offsets into the data. The writer never fills the
//! ring: one byte always stays free, so equal indices mean empty. Every
//! index the other side writes is checked to be inside the data before it
//! is used; a ring whose host index is out of bounds is broken, and the
//! channel with it.

use crate::mem::Shared;
use crate::packet::{Header, HEADER_BYTES, TRAILER_BYTES};

pub const PAGE: usize = 4096;

const WRITE_INDEX: usize = 0;
const READ_INDEX: usize = 4;
const INTERRUPT_MASK: usize = 8;
const PENDING_SEND_SZ: usize = 12;
const FEATURE_BITS: usize = 64;
/// The writer honours pending_send_sz (VMBUS_RING_BUFFER_FEAT_PENDING_SZ).
const FEAT_PENDING_SZ: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RingError {
    /// No room for the packet now; the host has not read enough yet.
    Full,
    /// The host's index is past the ring's data: the ring is broken.
    Broken,
    /// A packet the host wrote does not fit its own lengths, or the ring.
    Malformed,
    /// A packet larger than the buffer it was to be read into.
    TooBig,
}

/// One ring: its header page at `at` in the shared pages, then `pages - 1`
/// data pages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ring {
    at: usize,
    len: u32,
}

impl Ring {
    /// A ring of `pages` pages (at least 2) at byte `at`.
    pub const fn new(at: usize, pages: usize) -> Self {
        assert!(pages >= 2);
        Self {
            at,
            len: ((pages - 1) * PAGE) as u32,
        }
    }

    pub const fn data_len(&self) -> u32 {
        self.len
    }

    /// The header as the guest leaves it before the channel opens: empty,
    /// unmasked, nothing pending, and pending_send_sz honoured.
    pub fn init(&self, m: &mut impl Shared) {
        m.write(self.at, &[0u8; PAGE]);
        m.store32(self.at + FEATURE_BITS, FEAT_PENDING_SZ);
    }

    fn data(&self, i: u32) -> usize {
        self.at + PAGE + i as usize
    }

    fn index(&self, m: &impl Shared, field: usize) -> Result<u32, RingError> {
        let i = m.load32(self.at + field);
        if i >= self.len {
            return Err(RingError::Broken);
        }
        Ok(i)
    }

    /// Bytes from `from` up to `to`, around the ring.
    fn ahead(&self, from: u32, to: u32) -> u32 {
        (to + self.len - from) % self.len
    }

    fn copy_in(&self, m: &mut impl Shared, at: u32, data: &[u8]) -> u32 {
        let first = data.len().min((self.len - at) as usize);
        m.write(self.data(at), &data[..first]);
        m.write(self.data(0), &data[first..]);
        (at + data.len() as u32) % self.len
    }

    fn copy_out(&self, m: &impl Shared, at: u32, out: &mut [u8]) {
        let first = out.len().min((self.len - at) as usize);
        m.read(self.data(at), &mut out[..first]);
        m.read(self.data(0), &mut out[first..]);
    }

    /// The header of the packet at `read`, held to the `avail` bytes the
    /// host wrote: the packet and its trailer within them. A header reads
    /// as at least 16 bytes, so fewer than 24 available is refused here too.
    fn header_at(&self, m: &impl Shared, read: u32, avail: u32) -> Result<Header, RingError> {
        let mut raw = [0u8; HEADER_BYTES];
        self.copy_out(m, read, &mut raw);
        let h = Header::parse(&raw).ok_or(RingError::Malformed)?;
        if h.bytes() + TRAILER_BYTES > avail as usize {
            return Err(RingError::Malformed);
        }
        Ok(h)
    }

    /// Write one packet, the guest's: its header, its descriptor and its
    /// data (in parts), padded to 8 bytes, then the trailer. Whether to
    /// signal the host: it masked nothing and had read everything before,
    /// so it may be waiting (Linux's hv_ringbuffer_write).
    pub fn send(
        &self,
        m: &mut impl Shared,
        header: &Header,
        desc: &[u8],
        data: &[&[u8]],
    ) -> Result<bool, RingError> {
        let datalen: usize = data.iter().map(|d| d.len()).sum();
        let (hdr, total) = header.sized(desc.len(), datalen).ok_or(RingError::TooBig)?;
        let write = self.index(m, WRITE_INDEX)?;
        let read = self.index(m, READ_INDEX)?;
        // Room: everything unread aside, but one byte, so a full ring never
        // reads as empty.
        let room = self.len - 1 - self.ahead(read, write);
        if total as u32 > room {
            return Err(RingError::Full);
        }
        let mut at = self.copy_in(m, write, &hdr);
        at = self.copy_in(m, at, desc);
        for d in data {
            at = self.copy_in(m, at, d);
        }
        // The header and the descriptor are whole qwords (`sized`).
        let pad = (8 - datalen % 8) % 8;
        at = self.copy_in(m, at, &[0u8; 8][..pad]);
        let trailer = (u64::from(write) << 32).to_le_bytes();
        at = self.copy_in(m, at, &trailer[..TRAILER_BYTES]);
        m.barrier();
        m.store32(self.at + WRITE_INDEX, at);
        m.barrier();
        if m.load32(self.at + INTERRUPT_MASK) != 0 {
            return Ok(false);
        }
        Ok(m.load32(self.at + READ_INDEX) == write)
    }

    /// Whether a packet waits.
    pub fn pending(&self, m: &impl Shared) -> Result<bool, RingError> {
        Ok(self.index(m, WRITE_INDEX)? != self.index(m, READ_INDEX)?)
    }

    /// Read one packet the host wrote into `out`: its header and the bytes
    /// it occupies (header, descriptor, data, padding), or None when the
    /// ring is empty. The read index moves on past its trailer. Whether to
    /// signal the host is the second value: it said it waits for room
    /// (pending_send_sz) and now has it.
    pub fn recv(
        &self,
        m: &mut impl Shared,
        out: &mut [u8],
    ) -> Result<Option<(Header, usize, bool)>, RingError> {
        let write = self.index(m, WRITE_INDEX)?;
        let read = self.index(m, READ_INDEX)?;
        let avail = self.ahead(read, write);
        if avail == 0 {
            return Ok(None);
        }
        m.barrier();
        let h = self.header_at(m, read, avail)?;
        let bytes = h.bytes();
        if bytes > out.len() {
            return Err(RingError::TooBig);
        }
        self.copy_out(m, read, &mut out[..bytes]);
        let next = (read + (bytes + TRAILER_BYTES) as u32) % self.len;
        m.barrier();
        m.store32(self.at + READ_INDEX, next);
        m.barrier();
        let wanted = m.load32(self.at + PENDING_SEND_SZ);
        let room = self.len - 1 - self.ahead(next, write);
        let before = self.len - 1 - avail;
        let signal = wanted != 0 && room >= wanted && before < wanted;
        Ok(Some((h, bytes, signal)))
    }

    /// Drop the packet at the read index unread: for one too big to take.
    pub fn skip(&self, m: &mut impl Shared) -> Result<(), RingError> {
        let write = self.index(m, WRITE_INDEX)?;
        let read = self.index(m, READ_INDEX)?;
        let h = self.header_at(m, read, self.ahead(read, write))?;
        let next = (read + (h.bytes() + TRAILER_BYTES) as u32) % self.len;
        m.store32(self.at + READ_INDEX, next);
        Ok(())
    }
}
