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

//! How many edits apart two names are, a swapped pair counted as one.

use super::{MAX_EDITS, MAX_LEN};

/// Edit distance between two ASCII names, stopping once it exceeds `MAX_EDITS`.
///
/// Counts a transposition as one edit rather than two, which matters more here
/// than it looks. Under plain Levenshtein `hlep` is two edits from `help` and
/// also two from `grep`, so the tie is broken by whichever happens to be
/// listed first and the shell confidently proposes the wrong command. Swapping
/// a pair of letters is the most common way anyone mistypes, so it is the one
/// case worth spending a row of state on.
///
/// Three rows rather than a full matrix: a transposition reaches back two, and
/// nothing reaches further.
pub fn distance(a: &[u8], b: &[u8]) -> usize {
    if a.len() > MAX_LEN || b.len() > MAX_LEN {
        return usize::MAX;
    }
    if a.len().abs_diff(b.len()) > MAX_EDITS {
        return usize::MAX;
    }
    let mut prev2 = [0usize; MAX_LEN + 1];
    let mut prev = [0usize; MAX_LEN + 1];
    let mut cur = [0usize; MAX_LEN + 1];
    for (j, slot) in prev.iter_mut().enumerate().take(b.len() + 1) {
        *slot = j;
    }
    for i in 1..=a.len() {
        cur[0] = i;
        let mut best = cur[0];
        for j in 1..=b.len() {
            let sub = usize::from(a[i - 1] != b[j - 1]);
            let mut d = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + sub);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d = d.min(prev2[j - 2] + 1);
            }
            cur[j] = d;
            best = best.min(d);
        }
        // Every remaining row can only grow, so a row already past the limit
        // settles the answer.
        if best > MAX_EDITS {
            return usize::MAX;
        }
        prev2 = prev;
        prev = cur;
        cur = [0usize; MAX_LEN + 1];
    }
    prev[b.len()]
}
