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

//! What a connect is told, from where its stream has got to, and the
//! socket set back as it was when the stream did not open.

use super::super::sock;
use super::drive::{handle, Drove};
use super::table;
use crate::linux::abi::errno::{self, EAGAIN, EISCONN};

/// What connect answers for where its stream got to. `opening` is what a
/// non-blocking connect is told while it opens.
pub fn answer(id: u32, drove: Drove, nonblock: bool, opening: i64) -> u64 {
    match drove {
        Drove::Opened => errno::ok(0),
        Drove::Waiting => errno::fail(if nonblock { opening } else { EAGAIN }),
        Drove::Failed(e) => {
            reset(id);
            errno::fail(e)
        }
        /* Open since an earlier look, which a fresh connect never finds. */
        Drove::Same => errno::fail(EISCONN),
    }
}

/// Socket `id` as it was before its connect, which failed: unconnected,
/// and free to connect again, as Linux leaves it.
fn reset(id: u32) {
    let h = handle(id);
    sock::with(|t| {
        t.get_mut(id).map(|s| (s.svc, s.remote, s.connected, s.error) = (None, None, false, 0))
    });
    if let Some(h) = h {
        drop(table::remove(h));
    }
}
