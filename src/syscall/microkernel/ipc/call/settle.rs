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
//! late call or the starvation logged, and the display bench marks.
//!
//! On a timeout the pending entry is NOT removed: the server got the request
//! and will still reply to it; the redirect pairs replies to
//! callers strictly by FIFO position, so consuming an entry out of order
//! shifts every later reply onto the wrong caller, stamped with that
//! caller's own token. One timeout then desyncs the server's whole reply
//! stream and the misdeliveries cause further timeouts. Left in place, the
//! entry is popped in order and the late reply lands on this (no longer
//! waiting) inbox, where the correlation check discards it. Entries are only
//! removed when the send itself failed, where the server never saw a
//! request, and by clear_pid when either side dies.
//!
//! A negative receive after a successful send got no answer. If the caller's
//! own deadline, shorter than the kernel's default, ran out first, the call
//! was late (verdict.rs). A call that waited the full default is starvation,
//! and so is a pair whose late calls span the default with no answer in time
//! (silence.rs): the server is wedged or the reply path lost it.

use super::marks::display_marks;
use super::silence::Pair;
use super::silence_table;
use super::verdict::{no_answer, NoAnswer, DEFAULT_MS};
use crate::process::accounting;
use crate::services::registry::ServiceEndpoint;
use crate::syscall::microkernel::errnos::ERRNO_TIMEDOUT;

static STARVED: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.call");

/// `call` is the request's address and length and the reply's address;
/// `waited` the caller's timeout (0: the default) and the time it waited.
pub(super) fn settle(
    to: Option<&ServiceEndpoint>,
    pid: u32,
    call: (u64, usize, u64),
    rc: i64,
    waited: (u64, u64),
) {
    if rc >= 0 {
        accounting::bump(pid, accounting::Kind::IpcRx);
    }
    let Some(endpoint) = to else { return };
    let pair = Pair { caller: pid, port: endpoint.port };
    if rc >= 0 {
        silence_table::answered(pair);
    } else {
        match no_answer(rc == ERRNO_TIMEDOUT, waited.0, DEFAULT_MS, waited.1) {
            // A late call alone says nothing: a poll may miss. Only a pair
            // silent the default's length is reported (silence.rs).
            NoAnswer::Late => silence_table::report(&STARVED, pair, &endpoint.name, waited.1),
            NoAnswer::Starved => STARVED.starved(&endpoint.name, rc),
        }
    }
    display_marks(&endpoint.name, call.0, call.1, call.2, rc);
}
