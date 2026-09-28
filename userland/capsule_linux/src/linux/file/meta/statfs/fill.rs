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

/* struct statfs, from the mount table and the room on the mount. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::mounts;
use super::calls::{BLOCKS, BSIZE, FREE};

const STATFS: usize = 120;

/* f_flags: ST_VALID, which Linux always sets, and the mount's options. */
const ST_VALID: u64 = 0x20;

const OPTS: [(&str, u64); 5] =
    [("ro", 1), ("nosuid", 2), ("nodev", 4), ("noexec", 8), ("relatime", 0x1000)];

/*
 * The mount's type and flags from the family's mount table; the sizes are
 * the ones the personality declares for every mount.
 */
pub(super) fn fill(guest: &Guest, path: &[u8], out: u64) -> u64 {
    let (id, _, magic) = mounts::of(path);
    let opts = mounts::MOUNTS.iter().find(|m| m.0 == id).map_or("", |m| m.4);
    let mut buf = [0u8; STATFS];
    put(&mut buf, 0, magic); /* f_type */
    put(&mut buf, 8, BSIZE); /* f_bsize */
    put(&mut buf, 16, BLOCKS); /* f_blocks */
    put(&mut buf, 24, FREE); /* f_bfree */
    put(&mut buf, 32, FREE); /* f_bavail */
    put(&mut buf, 56, u64::from(id)); /* f_fsid */
    put(&mut buf, 64, 255); /* f_namelen, the vfs path limit */
    put(&mut buf, 72, BSIZE); /* f_frsize */
    put(&mut buf, 80, flags(opts)); /* f_flags */
    match guest.write(out, &buf) {
        n if n < 0 => errno::fail(errno::EFAULT),
        _ => errno::ok(0),
    }
}

fn put(buf: &mut [u8; STATFS], at: usize, v: u64) {
    buf[at..at + 8].copy_from_slice(&v.to_le_bytes());
}

pub(super) fn flags(opts: &str) -> u64 {
    let set = |name: &str| opts.split(',').any(|o| o == name);
    OPTS.iter().filter(|(name, _)| set(name)).fold(ST_VALID, |f, (_, bit)| f | bit)
}
