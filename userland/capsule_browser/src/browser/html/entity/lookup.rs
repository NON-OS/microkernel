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

use core::cmp::Ordering;

use super::names::{CHUNKS, LEGACY};

/// The characters a named reference stands for, the name given without its
/// semicolon: the first, and a second for the few names that expand to two
/// (NUL otherwise). None for a name the table does not hold.
pub fn lookup(name: &str) -> Option<(char, char)> {
    let key = name.as_bytes();
    let before = |c: &&[(&str, char, char)]| c.last().is_some_and(|l| order(l.0, key).is_lt());
    let chunk = CHUNKS.get(CHUNKS.partition_point(before))?;
    let at = chunk.binary_search_by(|row| order(row.0, key)).ok()?;
    Some((chunk[at].1, chunk[at].2))
}

/// The length of the longest name a page may write without its semicolon
/// that `run` starts with, or 0. Every such name sorts at or before `run`
/// and after anything with a smaller first letter, so one search finds
/// where to start and the walk back stays within one letter's names.
pub fn legacy_prefix(run: &[u8]) -> usize {
    let Some(&first) = run.first() else {
        return 0;
    };
    let end = LEGACY.partition_point(|n| order(n, run).is_le());
    LEGACY[..end]
        .iter()
        .rev()
        .take_while(|n| n.as_bytes()[0] == first)
        .filter(|n| run.starts_with(n.as_bytes()))
        .map(|n| n.len())
        .max()
        .unwrap_or(0)
}

/// Byte order of a table name against a key; a plain loop, since the names
/// are a few bytes long and a call into `memcmp` costs more than the compare.
fn order(name: &str, key: &[u8]) -> Ordering {
    let name = name.as_bytes();
    for (a, b) in name.iter().zip(key) {
        if a != b {
            return a.cmp(b);
        }
    }
    name.len().cmp(&key.len())
}

/// The longest name a page may write without its semicolon ("middot").
pub const LEGACY_MAX: usize = 6;

/// The longest name in the table ("CounterClockwiseContourIntegral").
pub const NAME_MAX: usize = 31;
