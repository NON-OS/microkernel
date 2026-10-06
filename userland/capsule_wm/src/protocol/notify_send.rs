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

//! What a failed lifecycle notification means for its subscriber.
//!
//! Any failed send dropped the subscriber for good. The desktop shell is the
//! subscriber that matters, and its inbox takes every pointer motion the
//! input router mirrors to it, so a burst of motion as a window opened could
//! fill it: the kernel answers EBUSY, the shell lost its subscription, and it
//! never subscribes again once it has. From then on the dock never marked an
//! app opened or closed. A full inbox is now waited on (the shell drains it
//! when it runs) and the event sent again; only an inbox that is gone or
//! refused drops the subscriber.

/// The kernel's EBUSY: the receiver's inbox is full.
pub const EBUSY: i64 = -16;

/// How many times a notification is offered to a full inbox, with a yield
/// between, before this one event is given up on (the subscription stays).
pub const BUSY_TRIES: u32 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sent {
    /// Delivered.
    Done,
    /// The inbox is full: yield and send again.
    Again,
    /// The inbox is gone or refuses this sender: drop the subscriber.
    Gone,
}

pub fn sent(rc: i64) -> Sent {
    match rc {
        0.. => Sent::Done,
        EBUSY => Sent::Again,
        _ => Sent::Gone,
    }
}

/// Offer one notification through `send` (one `mk_ipc_send_to_pid`) and
/// `wait` (one yield): true when the subscriber is to be dropped.
pub fn offer(mut send: impl FnMut() -> i64, mut wait: impl FnMut()) -> bool {
    for attempt in 0..BUSY_TRIES {
        match sent(send()) {
            Sent::Done => return false,
            Sent::Gone => return true,
            Sent::Again if attempt + 1 < BUSY_TRIES => wait(),
            Sent::Again => {}
        }
    }
    false
}
