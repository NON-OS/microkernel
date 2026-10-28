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

//! How long an open waits for its network, as an install's does: a network
//! that has not come up yet (no default read, no service, no directory, no
//! link, no circuit) is asked again a second later, until OPEN_MS from the
//! connect, nonos_route_link's own bound, and the connect is then
//! ENETUNREACH. A destination that refuses is never asked again here.

use super::super::anon_ops::{
    E_BAD_LEN, E_DIRECTORY_STALE, E_NO_CIRCUIT, E_NO_DIRECTORY, E_NO_LINK, E_NO_PATH, E_NO_TCP,
    E_TABLE_FULL,
};
use super::super::{NOT_YET_GAP_MS, OPEN_MS};

/// When an open that began at `since` gives up.
pub fn until(since: i64) -> i64 {
    since.saturating_add(OPEN_MS)
}

/// When to ask again after finding the network not ready at `now`, never
/// past `until`; None once `until` has come, when the open is refused.
pub fn again(until: i64, now: i64) -> Option<i64> {
    (now < until).then(|| now.saturating_add(NOT_YET_GAP_MS as i64).min(until))
}

/// Why net.anon opened no stream, for an answer worth asking again: its
/// network has not come up yet. None for every other answer.
pub fn not_ready(status: u16) -> Option<&'static str> {
    Some(match status {
        E_NO_TCP => "net.anon has no network link yet",
        E_NO_DIRECTORY => "no relay directory has been fetched yet",
        E_DIRECTORY_STALE => "its relay directory is out of date",
        E_NO_PATH => "no path through the relays can be chosen yet",
        E_NO_LINK => "no link to a guard relay is up yet",
        E_NO_CIRCUIT => "no circuit is built yet",
        _ => return None,
    })
}

/// Why net.anon opened no stream, for an answer not worth asking again, or
/// for no answer at all.
pub fn refused(status: Option<u16>) -> &'static str {
    match status {
        Some(E_TABLE_FULL) => "this program holds as many streams as net.anon gives one",
        Some(E_BAD_LEN) => "net.anon could not read the request",
        Some(_) => "net.anon refused the stream",
        None => "net.anon did not answer",
    }
}
