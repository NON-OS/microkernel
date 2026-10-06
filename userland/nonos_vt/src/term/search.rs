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

//! Finding text in the screen and its history. A match may run across a
//! wrapped line end, as the text it matches did when it was written.

use alloc::vec::Vec;

use super::search_match::matches;
use super::state::Term;
use super::types::Pos;

impl Term {
    /// The logical lines, as the absolute line each starts on, in order.
    fn logical_starts(&self) -> Vec<u64> {
        let mut starts = Vec::new();
        let mut n = self.first_line();
        while n <= self.last_line() {
            starts.push(n);
            let (_, next) = self.logical(n);
            n = next.max(n + 1);
        }
        starts
    }

    /// The next match after `from` (or before it when `back`), wrapping
    /// round the whole history once. Returns its first and last cells.
    pub fn find(&self, needle: &str, from: Pos, back: bool, case: bool) -> Option<(Pos, Pos)> {
        let needle: Vec<char> = needle.chars().collect();
        let starts = self.logical_starts();
        if starts.is_empty() || needle.is_empty() {
            return None;
        }
        let here = starts.iter().rposition(|&s| s <= from.line).unwrap_or(0);
        let n = starts.len();
        for step in 0..=n {
            let idx = if back { (here + n * 2 - step) % n } else { (here + step) % n };
            let (hay, _) = self.logical(starts[idx]);
            let hits = matches(&hay, &needle, case);
            let span = |i: usize| (hay[i].1, hay[i + needle.len() - 1].1);
            let pick = if step == 0 {
                if back {
                    hits.iter().rev().find(|&&i| hay[i].1 < from).copied()
                } else {
                    hits.iter().find(|&&i| hay[i].1 > from).copied()
                }
            } else if back {
                hits.last().copied()
            } else {
                hits.first().copied()
            };
            if let Some(i) = pick {
                return Some(span(i));
            }
        }
        None
    }
}
