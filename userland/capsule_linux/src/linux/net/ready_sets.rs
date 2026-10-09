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

//! The descriptor counts of poll and select, checked as Linux checks them,
//! and a select set narrowed to what is ready. Pure, so the host proofs hold
//! every refusal and every index into a set.

use crate::linux::abi::errno;

/// poll's nfds, an unsigned int: EINVAL above RLIMIT_NOFILE, `most`, so a
/// guest cannot have one call walk an unbounded array.
pub fn poll_count(raw: u64, most: u64) -> Result<u64, i64> {
    match u64::from(raw as u32) {
        n if n > most => Err(errno::EINVAL),
        n => Ok(n),
    }
}

/// select's nfds, an int: EINVAL when negative. Linux looks no further than
/// its descriptor table, and no descriptor here is `most` or above, so a
/// larger count is looked at only as far as that.
pub fn select_count(raw: u64, most: u64) -> Result<u64, i64> {
    u64::try_from(raw as u32 as i32).map(|n| n.min(most)).map_err(|_| errno::EINVAL)
}

/// The bytes of a set that `nfds` covers, in whole longs, as Linux copies them.
pub fn set_bytes(nfds: u64) -> usize {
    (nfds.div_ceil(64) * 8) as usize
}

/// Whether descriptor `fd` is named in `set`.
pub fn is_set(set: &[u8], fd: u64) -> bool {
    set.get((fd / 8) as usize).is_some_and(|b| b & (1 << (fd % 8)) != 0)
}

/// The first descriptor below `nfds` named in any of `sets` that is not
/// open. Linux answers select EBADF for it before it looks at any other.
pub fn first_closed(sets: &[&[u8]], nfds: u64, open: impl Fn(u64) -> bool) -> Option<u64> {
    (0..nfds).find(|&fd| sets.iter().any(|s| is_set(s, fd)) && !open(fd))
}

/// Clear every bit whose descriptor is not ready for `want`, and count the
/// bits left set.
pub fn narrow(set: &mut [u8], nfds: u64, ready: impl Fn(u64) -> u16, want: u16) -> u64 {
    let mut kept = 0;
    for fd in 0..nfds {
        let Some(byte) = set.get_mut((fd / 8) as usize) else {
            break;
        };
        let bit = 1u8 << (fd % 8);
        if *byte & bit == 0 {
            continue;
        }
        match ready(fd) & want {
            0 => *byte &= !bit,
            _ => kept += 1,
        }
    }
    kept
}
