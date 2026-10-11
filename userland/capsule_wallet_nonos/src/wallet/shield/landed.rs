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

/*
 * Refunds and reclaims that landed at a fresh address (B7, v1.4.52): the
 * addresses the Shield handed out once, past every account, so no account
 * list shows them. Each one's NOX is read, and any that holds some gets a
 * row, so the money is never out of sight. Pure: the reads are the
 * caller's.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// The newest handed addresses read each time the Shield opens.
pub const MOST_READ: usize = 16;

#[derive(Default)]
pub struct Landed {
    /// Index and address, newest first, once the Shield has listed them.
    pub list: Option<Vec<(u32, [u8; 20])>>,
    /// The next of `list` whose NOX is read.
    pub next: usize,
    /// Each handed index that holds NOX, and how much.
    pub found: Vec<(u32, u128)>,
}

/// OP_FRESH_ADDRESS mode 2's `handed` values, index|0x and 40 hex; a value
/// that does not read is skipped. Newest first, at most MOST_READ.
pub fn parse_handed<'a>(values: impl Iterator<Item = &'a str>) -> Vec<(u32, [u8; 20])> {
    let one = |v: &str| -> Option<(u32, [u8; 20])> {
        let (i, a) = v.split_once('|')?;
        let bytes = shield_wire::unhex(a).filter(|b| b.len() == 20)?;
        Some((i.parse().ok()?, bytes.try_into().ok()?))
    };
    let mut all: Vec<(u32, [u8; 20])> = values.filter_map(one).collect();
    all.sort_by(|a, b| b.0.cmp(&a.0));
    all.dedup_by_key(|h| h.0);
    all.truncate(MOST_READ);
    all
}

/// balanceOf's one word as NOX in wei; None past u128 or not one word.
pub fn balance_of(words: &[[u8; 32]]) -> Option<u128> {
    let [w] = words else { return None };
    let (high, low) = w.split_at(16);
    high.iter().all(|b| *b == 0).then(|| u128::from_be_bytes(low.try_into().unwrap_or([0; 16])))
}

pub fn line(index: u32, nox: &str) -> String {
    format!("fresh address {index}: {nox} NOX, a refund or a reclaim")
}
