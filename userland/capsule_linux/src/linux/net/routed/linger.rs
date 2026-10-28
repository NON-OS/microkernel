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

//! A stream closed with written bytes still queued keeps going until they
//! have gone, as Linux sends what was written before its FIN: a client that
//! writes a request and closes, or writes and exits, does not lose it. The
//! wait is bounded: the queue holds at most TX_MAX, and a stream that has
//! not carried it in LINGER_MS is ended then. What came back and was never
//! read is dropped at the close, as Linux drops it. Pure: the proofs hold it.

/// The longest a closed stream is kept to carry what its guest wrote.
pub const LINGER_MS: i64 = 10_000;

/// When a stream's socket is let go at `now`: until when it is kept to
/// carry what is queued, or None when it ends at once.
pub fn linger(queued: bool, now: i64) -> Option<i64> {
    queued.then(|| now.saturating_add(LINGER_MS))
}

/// Whether a closed stream ends now: nothing is left to carry, a write has
/// failed, or its time is up.
pub fn ends(queued: bool, until: i64, now: i64) -> bool {
    !queued || now >= until
}
