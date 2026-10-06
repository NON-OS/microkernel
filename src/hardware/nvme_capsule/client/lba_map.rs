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

//! 512-byte sector requests on a namespace addressed in its own LBAs.
//!
//! The block layer and everything above it count 512-byte sectors. The
//! driver capsule speaks the namespace's native LBAs: its read and write
//! carry an LBA and a count of LBAs, its capacity is in LBAs, and its
//! identify-namespace reply names the LBA size. A drive formatted with
//! 4096-byte LBAs (many laptop SSDs ship so) would read eight times the
//! bytes at eight times the offset if sectors were passed through, so the
//! kernel client maps them here. Pure, with no kernel imports, so
//! userland/nvme_proofs runs this file on the host.

/// The sector everything above the client counts in.
pub(crate) const SECTOR: usize = 512;

/// Whether the client can address a namespace of `lba_size`-byte LBAs: a
/// power of two, at least a sector, and no more than one capsule request
/// moves (`buffer` bytes), so a single LBA can always be read and written.
pub(crate) const fn addressable(lba_size: u32, buffer: u32) -> bool {
    lba_size.is_power_of_two() && lba_size as usize >= SECTOR && lba_size <= buffer
}

/// Bytes one capsule command may move, as the capsule computes it
/// (capsule_driver_nvme/src/nvm/geometry.rs, `max_transfer_bytes`): its data
/// buffer, or less when MDTS says so. MDTS is a power of two in units of the
/// 4 KiB minimum page the capsule requires; zero, or a value too large to
/// shift, is no limit.
pub(crate) const fn max_transfer_bytes(mdts: u8, buffer: u64) -> u64 {
    let shift = mdts as u32 + 12;
    if mdts == 0 || shift >= u64::BITS {
        return buffer;
    }
    let limit = 1u64 << shift;
    if limit < buffer {
        limit
    } else {
        buffer
    }
}

/// LBAs one capsule request may carry: what the capsule enforces as its
/// per-command ceiling. At least one for an addressable LBA size, since MDTS
/// never goes below one 4 KiB page.
pub(crate) const fn lbas_per_request(mdts: u8, lba_size: u32, buffer: u64) -> u64 {
    max_transfer_bytes(mdts, buffer) / lba_size as u64
}

/// The namespace's size in 512-byte sectors, or None if it does not fit.
pub(crate) const fn capacity_sectors(capacity_lbas: u64, lba_size: u32) -> Option<u64> {
    capacity_lbas.checked_mul(lba_size as u64 / SECTOR as u64)
}

/// A part of one LBA the request covers: read the whole LBA, and for a
/// write, change only these bytes and write it back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Partial {
    pub(crate) lba: u64,
    /// Where the covered bytes start inside the LBA.
    pub(crate) offset: usize,
    pub(crate) len: usize,
    /// Where they start in the caller's buffer.
    pub(crate) at: usize,
}

/// Whole LBAs the request covers, moved straight to or from the caller's
/// buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Whole {
    pub(crate) lba: u64,
    pub(crate) lbas: u64,
    /// Where they start in the caller's buffer.
    pub(crate) at: usize,
}

/// How a sector request maps to LBAs: an unaligned head, the whole LBAs, an
/// unaligned tail. A request inside one LBA is a head alone. At 512-byte
/// LBAs there is only ever the whole part.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Plan {
    pub(crate) head: Option<Partial>,
    pub(crate) body: Option<Whole>,
    pub(crate) tail: Option<Partial>,
}

/// Map `bytes` starting at 512-byte sector `sector` onto `lba_size`-byte
/// LBAs. None when the byte range overflows or the LBA size is not one
/// `addressable` takes.
pub(crate) fn plan(sector: u64, bytes: usize, lba_size: u32) -> Option<Plan> {
    if !lba_size.is_power_of_two() || (lba_size as usize) < SECTOR {
        return None;
    }
    let ls = lba_size as u64;
    let start = sector.checked_mul(SECTOR as u64)?;
    let end = start.checked_add(bytes as u64)?;
    let mut pos = start;
    let mut head = None;
    if !pos.is_multiple_of(ls) && pos < end {
        let offset = pos % ls;
        let len = (ls - offset).min(end - pos);
        head = Some(Partial { lba: pos / ls, offset: offset as usize, len: len as usize, at: 0 });
        pos += len;
    }
    let mut body = None;
    let whole = (end - pos) / ls;
    if whole != 0 {
        body = Some(Whole { lba: pos / ls, lbas: whole, at: (pos - start) as usize });
        pos += whole * ls;
    }
    let mut tail = None;
    if pos < end {
        tail = Some(Partial {
            lba: pos / ls,
            offset: 0,
            len: (end - pos) as usize,
            at: (pos - start) as usize,
        });
    }
    Some(Plan { head, body, tail })
}

/// The capsule requests a run of whole LBAs is cut into: (first LBA, count,
/// offset of its bytes from the run's first byte), each at most `per`
/// LBAs.
pub(crate) fn chunks(
    whole: Whole,
    per: u64,
    lba_size: u32,
) -> impl Iterator<Item = (u64, u64, usize)> {
    let per = per.max(1);
    let mut done = 0u64;
    core::iter::from_fn(move || {
        if done >= whole.lbas {
            return None;
        }
        let n = (whole.lbas - done).min(per);
        let item = (whole.lba + done, n, (done * lba_size as u64) as usize);
        done += n;
        Some(item)
    })
}
