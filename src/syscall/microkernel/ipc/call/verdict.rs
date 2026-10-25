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

//! Which deadline a call with no answer ran out on.
//!
//! A caller may bound its own wait below the kernel's default (net.core
//! polls its NIC driver for 8 ms). When that bound runs out first, the
//! server may well be serving and its reply simply lands after the caller
//! stopped waiting: the call was late. Only a call that waited the kernel's
//! full default, or longer, went unanswered in the sense of a starved or
//! wedged server.

/// The receive's bound for a caller that names none (timeout 0).
pub(super) const DEFAULT_MS: u64 = 5000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum NoAnswer {
    /// The caller's own, shorter deadline ran out first.
    Late,
    /// The call waited the kernel's default or longer.
    Starved,
}

/// The receive's bound: the caller's own, or `default_ms` for 0.
pub(super) const fn wait_ms(caller_ms: u64, default_ms: u64) -> u64 {
    if caller_ms == 0 {
        default_ms
    } else {
        caller_ms
    }
}

/// A call that got no answer after `elapsed_ms`, its caller having asked
/// for `caller_ms` (0: the default). Late only when the receive timed out:
/// one that failed at once, with no inbox or a bad buffer, ran out on no
/// deadline. Equal to the default is starved.
pub(super) const fn no_answer(
    timed_out: bool,
    caller_ms: u64,
    default_ms: u64,
    elapsed_ms: u64,
) -> NoAnswer {
    if timed_out && wait_ms(caller_ms, default_ms) < default_ms && elapsed_ms < default_ms {
        NoAnswer::Late
    } else {
        NoAnswer::Starved
    }
}

/// A message dropped from a reply inbox was a late reply to an earlier call,
/// not a forgery: every real reply carries its call's nonzero token, and
/// `mk_ipc_send` stamps 0.
pub(crate) const fn stale_reply(correlation: u64, want: u64) -> bool {
    correlation != 0 && correlation != want
}
