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

use super::exp_delay::exp_delay_ns;
use crate::crypto::random::fill_random;
use crate::topology;
use alloc::vec::Vec;

/// Mean delay a mix holds a packet for, in nanoseconds: the reference
/// client's default, so these packets look like everyone else's.
pub const MEAN_DELAY_NS: u64 = 15_000_000;

/// Per-hop delays, one for each hop of a route drawn from `seed`.
///
/// The delay is what actually mixes traffic: packets arriving together leave
/// apart, so an observer at both ends cannot pair them. Each one is drawn
/// afresh from an exponential distribution, as the reference client does. A
/// mix reads the field in nanoseconds; a fixed value, or one written in
/// milliseconds, has every mix pass the packet straight on in the order it
/// came, and timing alone then pairs what goes in with what comes out.
pub fn hop_delays(seed: &[u8; 32]) -> Option<Vec<[u8; 8]>> {
    let hops = match topology::route(seed) {
        Ok(hops) => hops.len(),
        Err(_) => crate::state::bootstrap_route(seed).len(),
    };
    let mut out = Vec::with_capacity(hops);
    for _ in 0..hops {
        let mut draw = [0u8; 8];
        fill_random(&mut draw).ok()?;
        out.push(exp_delay_ns(u64::from_le_bytes(draw), MEAN_DELAY_NS).to_be_bytes());
    }
    Some(out)
}
