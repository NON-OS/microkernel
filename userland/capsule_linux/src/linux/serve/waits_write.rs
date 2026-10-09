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

//! A write to a blocking pipe, which Linux answers only once all of it is in:
//! a longer one than the pipe has room for waits for the reader, repeatedly.

use crate::linux::abi::{errno, nr};
use crate::linux::call;
use crate::linux::guest::{Blocked, Guest};

use super::waits_iov::{all, so_far, vector};

/// The answer once the write is done, None while it waits. What goes in is
/// counted in `done`, and the next try starts past it.
pub fn carry_on(guest: &mut Guest, wait: &mut Blocked) -> Option<u64> {
    let a = wait.args;
    let pieces = match wait.nr {
        nr::WRITEV => match vector(guest, a[1], a[2]) {
            Ok(pieces) => pieces,
            Err(failed) => return Some(so_far(wait, failed)),
        },
        _ => alloc::vec![(a[1], a[2])],
    };
    let whole = pieces.iter().fold(0u64, |t, p| t.saturating_add(p.1));
    /* Nothing to put in: the plain call answers, refusals included. */
    if whole == 0 {
        return Some(call::write(guest, a[0], a[1], 0));
    }
    let mut skip = wait.done;
    for (base, len) in pieces {
        if skip >= len {
            skip -= len;
            continue;
        }
        let want = len - skip;
        let got = call::write(guest, a[0], base.wrapping_add(skip), want);
        skip = 0;
        if (got as i64) < 0 {
            /* EAGAIN waits, unless a non-blocking write already put some in. */
            let waits = got == errno::fail(errno::EAGAIN) && (wait.done == 0 || all(guest, a[0]));
            return (!waits).then(|| so_far(wait, got));
        }
        wait.done += got;
        if got < want {
            break;
        }
    }
    (wait.done >= whole || !all(guest, a[0])).then(|| errno::ok(wait.done))
}
