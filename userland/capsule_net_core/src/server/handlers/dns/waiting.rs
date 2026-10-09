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

//! One DNS lookup whose caller waits, and what the serve loop found when it
//! looked at it. Kept free of smoltcp and the kernel, as `pending` is.
//!
//! A caller is blocked in its call until it is answered, so it has at most one
//! lookup waiting. One that gave up and calls again still has a reply owed to
//! the call it gave up on, and the kernel hands a caller's replies to its
//! calls in order; so that owed reply is sent first (`take_for`), before
//! anything that answers the new call.

/// Lookups that may wait at once; one more is refused at once.
pub const PENDING_MAX: usize = 8;

/// One lookup whose caller is waiting.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Waiting<Q> {
    pub pid: u32,
    pub request_id: u32,
    pub query: Q,
    /// Uptime at which the caller is told the lookup failed.
    pub deadline_ms: i64,
}

/// What the serve loop found when it looked at a waiting lookup.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Look {
    /// No answer yet; keep waiting until the deadline.
    Pending,
    /// The caller was answered; the slot is free.
    Answered,
}
