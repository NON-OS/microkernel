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

//! Leaving the guest's tree by every path shape Linux accepts.
//!
//! The target is a file that exists outside /linux on every image. Opening it
//! at all is the escape; what the personality answers instead only has to be
//! an errno.

use crate::fs_paths::opened;
use crate::report::{Report, Seen};
use crate::sys::{call, AT_FDCWD, CHDIR, CLOSE, GETCWD, OPEN, OPENAT, RENAME, SYMLINK};

/// Outside the guest's tree on every store image this repo packs.
pub const OUTSIDE: &str = "capsules/std_proof.elf";

pub(crate) const O_RDONLY: u64 = 0;
const O_DIRECTORY: u64 = 0o200000;

pub fn scan(r: &mut Report) {
    let up = "../".repeat(16);
    crate::fs_paths::opens(r, &up);
    let root = call(OPEN, [p("/\0"), O_RDONLY | O_DIRECTORY, 0, 0, 0, 0]);
    let rel = format!("{up}{OUTSIDE}\0");
    let rc = match root {
        fd if fd >= 0 => call(OPENAT, [fd as u64, p(&rel), O_RDONLY, 0, 0, 0]),
        e => e,
    };
    r.check("openat from a root fd", opened(rc));
    let _ = call(CLOSE, [root as u64, 0, 0, 0, 0, 0]);
    for _ in 0..16 {
        let _ = call(CHDIR, [p("..\0"), 0, 0, 0, 0, 0]);
    }
    let mut cwd = [0u8; 64];
    let n = call(GETCWD, [cwd.as_mut_ptr() as u64, 64, 0, 0, 0, 0]);
    let at_root = n > 0 && cwd.starts_with(b"/\0");
    r.check("chdir above root", if at_root { Seen::Refused(0) } else { escaped("cwd", n) });
    let rc = call(OPENAT, [AT_FDCWD as u64, p(&format!("{OUTSIDE}\0")), O_RDONLY, 0, 0, 0]);
    r.check("relative after chdir", opened(rc));
    let rc = call(SYMLINK, [p("/\0"), p("/tmp/up\0"), 0, 0, 0, 0]);
    r.check("symlink to root", opened(rc));
    let dest = format!("/{up}nonos/linux/apps/pwned\0");
    let rc = call(RENAME, [p("/tmp\0"), p(&dest), 0, 0, 0, 0]);
    r.check("rename across the root", opened(rc));
}

pub(crate) fn p(s: &str) -> u64 {
    s.as_ptr() as u64
}

pub(crate) fn escaped(what: &str, rc: i64) -> Seen {
    Seen::Escaped(format!("{what} succeeded with {rc}"))
}
