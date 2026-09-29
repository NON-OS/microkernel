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

/* What stat says about a file, and a file's number and blocks. */

/* Bytes of a `struct stat` on this architecture. */
pub const STAT_LEN: usize = 144;

/* What stat says about a file, in the units struct stat takes. */
#[derive(Clone, Copy, Default)]
pub struct Meta {
    pub mode: u32,
    pub size: u64,
    pub ino: u64,
    pub nlink: u64,
    pub rdev: u64,
    pub dev: u64,
    /*
     * Wall-clock milliseconds. The store keeps one time; a time the family
     * set (held/times.rs) stands in for it until the next write.
     */
    pub mtime_ms: u64,
    pub atime_ms: u64,
}

/*
 * A file's number, stable for its path and never zero. Distinct numbers are
 * how a loader tells two libraries apart; zero for every file made each
 * dlopen after the first hand back the library already loaded.
 */
pub fn inode(path: &[u8]) -> u64 {
    let fold = path
        .iter()
        .fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(*b)).wrapping_mul(0x100_0000_01b3));
    fold | 1
}

/* st_blocks as tmpfs counts them: whole pages, in 512-byte units. */
pub fn blocks(size: u64) -> u64 {
    size.div_ceil(4096) * 8
}
