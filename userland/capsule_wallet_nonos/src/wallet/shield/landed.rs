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

//! Refunds and reclaims landed at once-only addresses (B7, v1.4.56): the due
//! list's reads and the restore scan's, one address per settle request,
//! spaced by a random few seconds. Pure: reads and randomness are the caller's.

use alloc::vec::Vec;

use super::due::Due;
use super::fresh_move::Moving;
use super::landed_held::Reading;
use super::scan::Scan;

/// The spacing between two reads: SPACE_MS plus up to SPACE_MS more.
pub const SPACE_MS: u64 = 2_000;
/// The most addresses one batch from the Shield may carry.
pub const BATCH_MAX: usize = 64;

#[derive(Default)]
pub struct Landed {
    pub due: Due,
    pub scan: Scan,
    /// When the next read may go out (uptime ms).
    pub next_ms: u64,
    pub reading: Option<Reading>,
    /// The scan's batch was asked of the Shield as a job.
    pub derive_asked: bool,
    /// NOX at a landed address being moved into the Shield.
    pub moving: Option<Moving>,
}

/// The wait before the next read, from a random 16-bit draw.
pub fn spacing(draw: u16) -> u64 {
    SPACE_MS + (draw as u64 * SPACE_MS) / u16::MAX as u64
}

/// The Shield's `handed` values, index|0x and 40 hex, in index order, each
/// index once, at most BATCH_MAX; a value that does not read is skipped.
pub fn parse_handed<'a>(values: impl Iterator<Item = &'a str>) -> Vec<(u32, [u8; 20])> {
    let one = |v: &str| -> Option<(u32, [u8; 20])> {
        let (i, a) = v.split_once('|')?;
        let bytes = shield_wire::unhex(a).filter(|b| b.len() == 20)?;
        Some((i.parse().ok()?, bytes.try_into().ok()?))
    };
    let mut all: Vec<(u32, [u8; 20])> = values.filter_map(one).take(BATCH_MAX * 4).collect();
    all.sort_by_key(|h| h.0);
    all.dedup_by_key(|h| h.0);
    all.truncate(BATCH_MAX);
    all
}

/// balanceOf's one word as NOX in wei; None past u128 or not one word.
pub fn balance_of(words: &[[u8; 32]]) -> Option<u128> {
    let [w] = words else { return None };
    let (high, low) = w.split_at(16);
    high.iter().all(|b| *b == 0).then(|| u128::from_be_bytes(low.try_into().unwrap_or([0; 16])))
}
