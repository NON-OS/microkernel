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

/* `statx`, which a current libc reaches for before it tries `stat`. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::stat::meta_at;
use super::statbuf::blocks;

const STATX: usize = 256;

/* Type, mode, nlink, uid, gid, times, ino, size and blocks: STATX_BASIC_STATS. */
const STATX_BASIC_STATS: u32 = 0x07ff;

pub fn statx(guest: &Guest, dirfd: u64, path: u64, flags: u64, out: u64) -> u64 {
    let m = match meta_at(guest, dirfd, path, flags) {
        Ok(m) => m,
        Err(e) => return errno::fail(e),
    };
    let mut buf = [0u8; STATX];
    let mut put = |at: usize, v: &[u8]| buf[at..at + v.len()].copy_from_slice(v);
    put(0, &STATX_BASIC_STATS.to_le_bytes());
    put(4, &4096u32.to_le_bytes()); /* stx_blksize */
    put(16, &(m.nlink as u32).to_le_bytes()); /* stx_nlink */
    put(28, &(m.mode as u16).to_le_bytes()); /* stx_mode */
    put(32, &m.ino.to_le_bytes()); /* stx_ino */
    put(40, &m.size.to_le_bytes()); /* stx_size */
    put(48, &blocks(m.size).to_le_bytes()); /* stx_blocks */
    let split = |ms: u64| ((ms / 1000) as i64, ((ms % 1000) * 1_000_000) as u32);
    /* atime, then ctime and mtime, which the store does not tell apart. */
    for (at, ms) in [(64, m.atime_ms), (96, m.mtime_ms), (112, m.mtime_ms)] {
        let (secs, nanos) = split(ms);
        put(at, &secs.to_le_bytes());
        put(at + 8, &nanos.to_le_bytes());
    }
    put(128, &((m.rdev >> 8) as u32 & 0xfff).to_le_bytes()); /* stx_rdev_major */
    put(132, &((m.rdev & 0xff) as u32).to_le_bytes()); /* stx_rdev_minor */
    put(140, &(m.dev as u32).to_le_bytes()); /* stx_dev_minor */
    match guest.write(out, &buf) {
        n if n < 0 => errno::fail(errno::EFAULT),
        _ => errno::ok(0),
    }
}
