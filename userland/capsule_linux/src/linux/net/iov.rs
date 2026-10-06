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

//! The byte vectors the message calls name: one buffer, or an iovec array.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub type Iov = Vec<(u64, u64)>;

/// The `count` entries of the iovec array at `at`, checked as Linux checks
/// them (`call::iovecs`).
pub fn read(guest: &Guest, at: u64, count: u64) -> Result<Iov, u64> {
    crate::linux::call::iovecs(guest, at, count).map_err(errno::fail)
}

/// Every length together; past what an address space holds it is that.
pub fn total(iov: &Iov) -> usize {
    iov.iter().fold(0usize, |t, &(_, len)| t.saturating_add(len as usize))
}

/// The message's bytes from `skip` on, at most `cap` of them.
pub fn gather(guest: &Guest, iov: &Iov, skip: usize, cap: usize) -> Result<Vec<u8>, u64> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    for &(base, len) in iov {
        let len = len as usize;
        let (from, to) = (skip.max(pos), pos.saturating_add(len).min(skip.saturating_add(cap)));
        if from < to {
            let part = guest.read(base.wrapping_add((from - pos) as u64), to - from);
            out.extend_from_slice(&part.ok_or(errno::fail(errno::EFAULT))?);
        }
        pos = pos.saturating_add(len);
    }
    Ok(out)
}

/// Put `bytes` into the message's buffers starting `skip` bytes in.
pub fn scatter(guest: &Guest, iov: &Iov, skip: usize, bytes: &[u8]) -> Result<(), u64> {
    let mut pos = 0usize;
    for &(base, len) in iov {
        let len = len as usize;
        let end = skip.saturating_add(bytes.len());
        let (from, to) = (skip.max(pos), pos.saturating_add(len).min(end));
        if from < to {
            let part = &bytes[from - skip..to - skip];
            if guest.write(base.wrapping_add((from - pos) as u64), part) < part.len() as i64 {
                return Err(errno::fail(errno::EFAULT));
            }
        }
        pos = pos.saturating_add(len);
    }
    Ok(())
}
