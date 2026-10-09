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

/* struct stat, x86_64 layout. */

use super::shape::{blocks, Meta, STAT_LEN};

/*
 * struct stat, x86_64 layout. Owner and group are root, as the ids the
 * personality reports are.
 */
pub fn build(m: &Meta) -> [u8; STAT_LEN] {
    let mut out = [0u8; STAT_LEN];
    let split = |ms: u64| (ms / 1000, (ms % 1000) * 1_000_000);
    put64(&mut out, 0, m.dev);
    put64(&mut out, 8, m.ino);
    put64(&mut out, 16, m.nlink);
    put32(&mut out, 24, m.mode);
    put64(&mut out, 40, m.rdev);
    put64(&mut out, 48, m.size);
    put64(&mut out, 56, 4096);
    put64(&mut out, 64, blocks(m.size));
    /* atime, then mtime and ctime, which the store does not tell apart. */
    for (at, ms) in [(72, m.atime_ms), (88, m.mtime_ms), (104, m.mtime_ms)] {
        let (secs, nanos) = split(ms);
        put64(&mut out, at, secs);
        put64(&mut out, at + 8, nanos);
    }
    out
}

fn put32(out: &mut [u8; STAT_LEN], at: usize, value: u32) {
    out[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

fn put64(out: &mut [u8; STAT_LEN], at: usize, value: u64) {
    out[at..at + 8].copy_from_slice(&value.to_le_bytes());
}
