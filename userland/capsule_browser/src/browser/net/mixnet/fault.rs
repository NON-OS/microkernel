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

//! Which way a call to a proxy failed, from what the kernel returned.
//!
//! Pure; the proofs hold it.

/* The kernel's answers that mean "not now" rather than "not there". */
const ERRNO_AGAIN: i64 = -11;
const ERRNO_BUSY: i64 = -16;
const ERRNO_TIMEDOUT: i64 = -110;

/// Why a call brought back no answer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fault {
    /// No answer in the wait, or the kernel holds too many of our calls to
    /// this proxy already (EBUSY) or its queue is full (EAGAIN). Asked again
    /// later, the frame is the same.
    Unanswered,
    /// The proxy is not there to ask: no such port, a dead service, or a
    /// call the kernel will not carry. Asking again changes nothing.
    Refused,
}

/// Which way a call that returned `rc`, a negative errno, failed.
pub fn fault(rc: i64) -> Fault {
    match rc {
        ERRNO_TIMEDOUT | ERRNO_BUSY | ERRNO_AGAIN => Fault::Unanswered,
        _ => Fault::Refused,
    }
}
