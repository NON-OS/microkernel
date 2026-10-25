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

/* ftruncate: the file a descriptor holds open, cut or grown. */

use super::super::advice::resize;
use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

pub fn ftruncate(guest: &mut Guest, fd: u64, len: u64) -> u64 {
    if (len as i64) < 0 {
        return errno::fail(errno::EINVAL);
    }
    let Some(entry) = guest.fds.get_mut(fd as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    match entry.kind {
        /*
         * Sizing a memfd records the size and nothing else. The pages appear
         * when the client maps it, because that is when their address is decided.
         */
        Kind::Memfd => {
            entry.size = len;
            errno::ok(0)
        }
        /* Linux answers EINVAL for anything but a regular file open to write. */
        /*
         * The descriptor learns the new length only once the copy has it: a
         * length refused EFBIG is not one the file has, and remembered it
         * reads back from fstat and sizes a later fallocate.
         */
        Kind::File if entry.writable && !super::super::super::super::synth::owns(&entry.path) => {
            let path = entry.path.clone();
            let done = resize(&path, len, true);
            if done == errno::ok(0) {
                entry.size = len;
            }
            done
        }
        _ => errno::fail(errno::EINVAL),
    }
}
