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

//! MADV_DONTNEED: each page of the span reads zero from then on, as Linux
//! gives private anonymous memory back.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Region};

/// Zeroes written a piece at a time.
const CHUNK: usize = 64 << 10;

/// The pieces of the guest's regions within [at, end), and whether any page
/// of it is not mapped, which Linux answers with ENOMEM once it has applied
/// the advice to the pages that are.
pub(super) fn parts(guest: &Guest, at: u64, end: u64) -> (Vec<Region>, bool) {
    let (mut out, mut hole, mut reach) = (Vec::new(), false, at);
    while reach < end {
        let Some(r) = guest.regions.iter().find(|r| r.at <= reach && reach < r.at + r.len) else {
            hole = true;
            reach = guest.regions.iter().map(|r| r.at).filter(|&s| s > reach).min().unwrap_or(end);
            continue;
        };
        let stop = (r.at + r.len).min(end);
        out.push(Region { at: reach, len: stop - reach, ..*r });
        reach = stop;
    }
    (out, hole)
}

/// What Linux would reload rather than zero, a file's bytes or a shared
/// mapping's, this capsule cannot, so any of it refuses the whole span before
/// a byte changes. A backed page is zeroed where it is and keeps its
/// protection; a reservation's touched pages are given back, to be filled
/// with zeroes when next touched.
pub(super) fn drop_pages(guest: &mut Guest, parts: &[Region]) -> u64 {
    if parts.iter().any(|p| p.kept) {
        return errno::fail(errno::EINVAL);
    }
    let zeros = alloc::vec![0u8; CHUNK];
    for p in parts {
        let rc = if p.backed { zero(guest, p, &zeros) } else { guest.drop_frames(p.at, p.len) };
        if rc < 0 {
            return errno::fail(errno::EFAULT);
        }
    }
    errno::ok(0)
}

fn zero(guest: &Guest, p: &Region, zeros: &[u8]) -> i64 {
    let mut at = p.at;
    while at < p.at + p.len {
        let n = (p.at + p.len - at).min(zeros.len() as u64);
        if guest.write(at, &zeros[..n as usize]) < 0 {
            return -1;
        }
        at += n;
    }
    0
}
