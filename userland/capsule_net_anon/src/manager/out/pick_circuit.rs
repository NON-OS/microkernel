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

//! Choosing which circuit a new stream goes down.

use crate::circuit::CircuitStage;
use crate::protocol::{CIRCUIT_DIRTY_SECONDS, CIRCUIT_FAILURES_MAX};

use super::super::state::Manager;

/*
 * Taking the first open circuit every time is what made browsing over the mixnet
 * exit roulette: one exit carried the whole session, and when that exit blocked
 * the port or sat on a blocklist every page failed with no way out of it. So this
 * prefers a circuit that has actually delivered payload, skips one that has
 * failed twice without ever delivering, retires one too old for new streams, and
 * spreads consecutive streams across what is left instead of stacking them.
 */

pub(super) fn pick(state: &mut Manager, now: u64) -> Option<usize> {
    let cursor = state.circuit_cursor;
    let best = scan(state, now, cursor, true).or_else(|| scan(state, now, cursor, false))?;
    state.circuit_cursor = best.wrapping_add(1);
    Some(best)
}

fn scan(state: &Manager, now: u64, cursor: usize, want_proven: bool) -> Option<usize> {
    let n = state.circuits.len();
    if n == 0 {
        return None;
    }
    (0..n)
        .map(|step| (cursor.wrapping_add(step)) % n)
        .find(|&i| usable(state, i, now) && state.circuits[i].proven == want_proven)
}

fn usable(state: &Manager, index: usize, now: u64) -> bool {
    let c = &state.circuits[index];
    c.stage == CircuitStage::Open
        && c.failures < CIRCUIT_FAILURES_MAX
        && now < c.opened_at.saturating_add(CIRCUIT_DIRTY_SECONDS)
}
