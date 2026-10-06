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

//! The bounds a TLS flight is read under through an anonymity network, as
//! `step::gather` keeps them between slices. A routed answer is read to the
//! close the proxy reports, never to a guess at how many records it holds.

use super::bounds::Bounds;

/* The most a flight may hold, on either kind of link. */
pub(super) const FLIGHT_MAX: usize = 24 * 1024;
/* The most an answer may hold: a batch of receipts, each with every log its
 * transaction emitted, runs far past a flight. */
pub(super) const ANSWER_MAX: usize = 512 * 1024;

/* Through an anonymity network: how long the far end may pause between the
 * pieces of one answer, and the most one answer may take in all. */
const ROUTED_QUIET_MS: u64 = 8_000;
const ROUTED_TOTAL_MS: u64 = 90_000;

pub(super) fn routed(patience_ms: u64) -> Bounds {
    Bounds {
        first_ms: patience_ms,
        quiet_ms: ROUTED_QUIET_MS,
        total_ms: ROUTED_TOTAL_MS,
        max: FLIGHT_MAX,
    }
}
