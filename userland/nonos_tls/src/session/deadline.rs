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

//! The whole handshake's budget, from the ClientHello's first byte to the
//! server's Finished. Each read also has its own quiet rule, but a peer that
//! sends one byte inside every quiet window would pass that rule forever:
//! this cuts it off. Past the end the handshake fails as Handshake, the
//! flight that never completed, so no caller's match has to change.

use nonos_libc::mk_uptime_ms;

use super::traits::SessionError;

/// How long a handshake may take, in milliseconds of uptime.
pub const HANDSHAKE_MS: i64 = 30_000;

/// The end of a handshake whose first byte goes now: HANDSHAKE_MS on, or
/// `until` when the caller's own end is sooner.
pub(crate) fn handshake_end(until: Option<i64>) -> i64 {
    let own = mk_uptime_ms().saturating_add(HANDSHAKE_MS);
    until.map_or(own, |u| u.min(own))
}

/// Ok while `end` is still ahead.
pub(crate) fn within(end: i64) -> Result<(), SessionError> {
    if mk_uptime_ms() < end {
        Ok(())
    } else {
        Err(SessionError::Handshake)
    }
}

/// How long the response after the handshake may take, in milliseconds of
/// uptime (A9f). Each read still has its quiet rule; this bounds the whole,
/// so a peer that sends a byte inside every quiet window ends here. Five
/// minutes carries a 64 MiB git pack over a slow relay.
pub const REPLY_MS: i64 = 300_000;

/// What a response past its budget ends as: named apart from a socket
/// that failed (plan/41 rule 2).
pub const REPLY_TOO_SLOW: SessionError = SessionError::TooSlow;
