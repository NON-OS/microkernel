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

/* copy_file_range. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::desc;
use super::super::super::rw::write_at;
use super::bytes::move_bytes;
use super::offset::read_offset;

pub fn copy_file_range(guest: &mut Guest, a: [u64; 6]) -> u64 {
    let (input, off_in, out, off_out, len, flags) = (a[0], a[1], a[2], a[3], a[4], a[5]);
    if flags != 0 {
        return errno::fail(errno::EINVAL);
    }
    let files =
        [input, out].map(|fd| guest.fds.get(fd as usize).filter(|f| f.is_open()).map(|f| f.kind));
    match files {
        [None, _] | [_, None] => return errno::fail(errno::EBADF),
        [Some(Kind::File), Some(Kind::File)] => {}
        [Some(Kind::Dir), _] | [_, Some(Kind::Dir)] => return errno::fail(errno::EISDIR),
        _ => return errno::fail(errno::EINVAL),
    }
    let target = &guest.fds[out as usize];
    if !target.writable || super::super::super::desc::appends(target) {
        return errno::fail(errno::EBADF);
    }
    let mut at_out = match read_offset(guest, off_out) {
        Ok(Some(at)) => at,
        Ok(None) => desc::pos(target),
        Err(e) => return e,
    };
    let start_out = at_out;
    let got = move_bytes(guest, input, off_in, len, |g, bytes| {
        let (n, end) = write_at(g, out, at_out, bytes)?;
        at_out = end;
        Ok(n)
    });
    if (got as i64) > 0 {
        let moved = at_out - start_out;
        if off_out == 0 {
            desc::set_pos(&mut guest.fds[out as usize], start_out + moved);
        } else if guest.write(off_out, &at_out.to_le_bytes()) < 8 {
            return errno::fail(errno::EFAULT);
        }
    }
    got
}
