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

//! How long a fetch may wait, in milliseconds rather than in ticks: under
//! load a tick took half a second, and a budget counted in ticks ran out on a
//! flight that had arrived. A mixnet holds every packet at every hop on
//! purpose, so its budgets are larger in proportion.

/*
 * Silent: without a byte arriving or the phase moving. Total: in all,
 * however steadily bytes arrive. Connect: for a connection to be accepted.
 * Idle: quiet after which a response with no length is taken as whole.
 * Reuse: for a request on a kept connection to draw its first byte.
 */
pub struct Budget {
    pub silent_ms: i64,
    pub total_ms: i64,
    pub connect_ms: i64,
    pub idle_ms: i64,
    pub reuse_ms: i64,
}

pub const DIRECT: Budget = Budget {
    silent_ms: 12_000,
    total_ms: 120_000,
    connect_ms: 8_000,
    idle_ms: 2_000,
    reuse_ms: 4_000,
};

pub const MIXNET: Budget = Budget {
    silent_ms: 180_000,
    total_ms: 900_000,
    connect_ms: 180_000,
    idle_ms: 24_000,
    reuse_ms: 48_000,
};

pub fn budget(mixnet: bool) -> &'static Budget {
    if mixnet {
        &MIXNET
    } else {
        &DIRECT
    }
}
