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

//! Reading what a socket holds now, and no longer than a bound.
//!
//! A fixed count of 4 KiB reads left bytes waiting behind a full burst and
//! spent reads on a socket that had long gone quiet. This reads until a read
//! comes back empty, the buffer reaches its cap, the far end has closed, or
//! the time is spent.

use alloc::vec::Vec;

use super::recv_kind::{Recv, Source};

/// The most one read is offered, which is what one reply can carry.
const CHUNK: usize = 32 * 1024;

/// What a drain took, whether it stopped because the cap was reached, and
/// whether the far end has finished: every byte it sent is now in the
/// buffer and nothing more will come.
pub struct Drained {
    pub got: usize,
    pub full: bool,
    pub closed: bool,
}

/// Append to `into` what `handle` holds, until a read is empty or closed,
/// `into` holds `cap` bytes, or `budget_ms` has passed. A lost reply is
/// asked for once more before the drain gives up on it for this call.
pub fn drain<S>(s: &mut S, handle: u32, into: &mut Vec<u8>, cap: usize, budget_ms: i64) -> Drained
where
    S: Source + ?Sized,
{
    let until = s.now_ms().saturating_add(budget_ms);
    let (mut got, mut asked_again) = (0, false);
    loop {
        let start = into.len();
        let room = cap.saturating_sub(start).min(CHUNK);
        if room == 0 {
            return Drained { got, full: true, closed: false };
        }
        into.resize(start + room, 0);
        let read = s.recv(handle, &mut into[start..]);
        let n = match read {
            Recv::Bytes(n) => n.min(room),
            Recv::Empty | Recv::Lost | Recv::Closed => 0,
        };
        into.truncate(start + n);
        got += n;
        if read == Recv::Closed {
            return Drained { got, full: false, closed: true };
        }
        let again = read == Recv::Lost && !asked_again;
        asked_again |= again;
        if (n == 0 && !again) || s.now_ms() >= until {
            return Drained { got, full: false, closed: false };
        }
    }
}
