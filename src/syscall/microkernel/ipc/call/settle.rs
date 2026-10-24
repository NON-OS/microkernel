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

//! What a call leaves once its receive returns: the reply counted, or the
//! starvation logged, and the display bench marks.
//!
//! On no answer, the pending entry is NOT removed on a timeout. The server received this
//! request and will still reply to it; the redirect pairs replies to
//! callers strictly by FIFO position, so consuming an entry out of order
//! shifts every later reply onto the wrong caller, stamped with that
//! caller's own token. One timeout then desyncs the server's whole reply
//! stream and the misdeliveries cause further timeouts. Left in place, the
//! entry is popped in order and the late reply lands on this (no longer
//! waiting) inbox, where the correlation check discards it. Entries are only
//! removed when the send itself failed, where the server never saw a
//! request, and by clear_pid when either side dies.
//!
//! A negative receive after a successful send is a served call that got no
//! answer: the server is wedged or the reply path lost the message.
//! Refusals log at the gates, so this line is specifically starvation.

use super::marks::display_marks;
use crate::process::accounting;
use crate::services::registry::ServiceEndpoint;

static STARVED: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.call");

/// `call` is the request's address and length and the reply's address.
pub(super) fn settle(to: Option<&ServiceEndpoint>, pid: u32, call: (u64, usize, u64), rc: i64) {
    if rc >= 0 {
        accounting::bump(pid, accounting::Kind::IpcRx);
    }
    let Some(endpoint) = to else { return };
    if rc < 0 {
        STARVED.starved(&endpoint.name, pid, rc, endpoint.pid);
    }
    display_marks(&endpoint.name, call.0, call.1, call.2, rc);
}
