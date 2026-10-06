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

//! When the node list is fetched.
//!
//! A fetched list is good for an hour (fetched.rs), after which no route is
//! built from it. The fetch ran only until the list first held a gateway and
//! an exit, so an hour into the boot every send failed as expired and nothing
//! fetched a new list. It now also runs once the fetched list is within
//! `REFRESH_BEFORE_MS` of expiring, so the replacement is installed while the
//! old one still routes; a fetch that keeps failing leaves the old list until
//! it expires, and then nothing is sent, which is the closed failure.
//!
//! Pure, and held in nym_topology_proofs.

/// How long before a fetched list expires the next fetch may start.
pub const REFRESH_BEFORE_MS: u64 = 10 * 60 * 1000;

/// Whether to fetch now. `gateways` and `exits` count what the held list
/// offers. `fetched_until` is the expiry of a list fetched from the API,
/// `None` for one compiled into the image or signed by an authority, which
/// are not replaced on a clock. `now` is the wall clock the expiry is set
/// against, `None` when it cannot be read.
pub fn fetch_due(gateways: usize, exits: usize, fetched_until: Option<u64>, now: Option<u64>) -> bool {
    if gateways == 0 || exits == 0 {
        return true;
    }
    match (fetched_until, now) {
        (Some(until), Some(now)) => now >= until.saturating_sub(REFRESH_BEFORE_MS),
        _ => false,
    }
}
