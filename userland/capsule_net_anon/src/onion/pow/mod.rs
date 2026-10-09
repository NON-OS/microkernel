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


//! Onion-service proof of work, v1 (proposal 327; the fork's hs_pow.c and
//! hs_client.c). A service under load publishes a seed and a suggested
//! effort in its descriptor; a client that solves an Equi-X puzzle bound to
//! that seed, the service's blinded key and an effort is queued ahead of
//! clients that did not.
//!
//! Solving is optional for the client and never required by the protocol:
//! a service that suggests effort still accepts an introduction without a
//! solution, it just serves it last. So a puzzle that cannot be solved in
//! time costs priority, not reachability.

mod params;
mod puzzle;

pub use params::{params, PowParams};
pub use puzzle::{extension, Puzzle, PowSolution, EXTENSION_BYTES};

/// CLIENT_MAX_POW_EFFORT: the highest effort this client will spend, however
/// much a descriptor suggests. At about 60 ms a solve on a desktop core and
/// one acceptable solution per effort/2 solves on average, this is minutes of
/// work, and the lookup deadline bounds it further.
pub const CLIENT_MAX_EFFORT: u32 = 10_000;
/// CLIENT_MIN_RETRY_POW_EFFORT: a retry raises the effort to at least this.
const CLIENT_MIN_RETRY_EFFORT: u64 = 8;
/// CLIENT_POW_EFFORT_DOUBLE_UNTIL: retries double the effort below this, and
/// raise it by half above it.
const CLIENT_DOUBLE_UNTIL: u64 = 1000;

/// The effort to solve at: the descriptor's suggestion, capped, then raised
/// once per introduction point that did not get us through. This is
/// hs_client.c's rule, so a client behaves as the service expects one to.
pub fn effort(suggested: u32, unreachable: u32) -> u32 {
    let mut effort = u64::from(suggested.min(CLIENT_MAX_EFFORT));
    for _ in 0..unreachable {
        if effort >= u64::from(CLIENT_MAX_EFFORT) {
            break;
        }
        effort = if effort < CLIENT_DOUBLE_UNTIL { effort << 1 } else { effort + effort / 2 };
        effort = effort.clamp(CLIENT_MIN_RETRY_EFFORT, u64::from(CLIENT_MAX_EFFORT));
    }
    effort as u32
}
