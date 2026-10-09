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

/* A descriptor on a made file, and a made path refused by name. */

use crate::linux::abi::errno;
use crate::linux::guest::Fd;

/* A descriptor on a made file: its bytes come from its path at each read. */
pub(super) fn made_file(path: &[u8], writing: bool, flags: u64) -> Fd {
    let mut fd = Fd::file(path.to_vec(), 0, None, writing);
    fd.handle =
        super::super::super::desc::fresh(false, super::super::super::flags::wants_read(flags));
    fd
}

pub(super) fn refuse(why: &str) -> u64 {
    let line = alloc::format!("[LINUX] refused {why}\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    errno::fail(errno::EACCES)
}
