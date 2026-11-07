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

//! Deterministic schedules: a seed gives the EC's ready time and the port
//! accesses an SMI lands before. Xorshift, so a failing seed replays.

use super::board::Board;
use std::collections::BTreeSet;

/// The EC's start, in ms after the capsule's first access: at once, slow,
/// and past three of the handshake's one-second waits.
pub const READY_MS: [u64; 5] = [0, 400, 1_500, 3_500, 8_000];

pub struct Schedule {
    pub ready_ms: u64,
    pub smi_at: BTreeSet<u64>,
}

fn next(x: &mut u64) -> u64 {
    *x ^= *x << 13;
    *x ^= *x >> 7;
    *x ^= *x << 17;
    *x
}

/// Up to six SMIs among the first `window` port accesses.
pub fn schedule(seed: u64, window: u64) -> Schedule {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let ready_ms = READY_MS[(next(&mut x) % READY_MS.len() as u64) as usize];
    let smis = next(&mut x) % 7;
    let smi_at = (0..smis).map(|_| 1 + next(&mut x) % window).collect();
    Schedule { ready_ms, smi_at }
}

impl Schedule {
    pub fn board(&self) -> Board {
        Board::new(self.ready_ms, self.smi_at.clone())
    }
}
