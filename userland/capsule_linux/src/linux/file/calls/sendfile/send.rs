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

/* sendfile. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::desc;
use super::super::super::rw::write_at;
use super::bytes::move_bytes;

pub fn sendfile(guest: &mut Guest, out: u64, input: u64, offset: u64, count: u64) -> u64 {
    let Some(kind) = guest.fds.get(out as usize).filter(|f| f.is_open()).map(|f| f.kind) else {
        return errno::fail(errno::EBADF);
    };
    if !matches!(kind, Kind::File | Kind::Stdout | Kind::Stderr) {
        return errno::fail(errno::EINVAL);
    }
    if kind == Kind::File && !guest.fds[out as usize].writable {
        return errno::fail(errno::EBADF);
    }
    move_bytes(guest, input, offset, count, |g, bytes| match kind {
        Kind::File => {
            let at = desc::pos(&g.fds[out as usize]);
            let (n, end) = write_at(g, out, at, bytes)?;
            desc::set_pos(&mut g.fds[out as usize], end);
            Ok(n)
        }
        _ => {
            let _ = nonos_libc::mk_debug(bytes.as_ptr(), bytes.len());
            Ok(bytes.len())
        }
    })
}
