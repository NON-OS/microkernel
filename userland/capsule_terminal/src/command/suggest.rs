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

//! "did you mean": the nearest command to something that was not one.
//!
//! A mistyped command is nearly always a typo or a name from another system,
//! and both have an obvious neighbour. Printing only that a name is unknown
//! makes the reader go and look it up; printing the neighbour usually ends the
//! problem on the spot.
//!
//! The distance is Levenshtein, bounded, over ASCII. Command names are short
//! and the candidate list is under a hundred, so the cost of this is invisible
//! next to spawning anything.

/// Longest name this will compare. Anything longer is not a mistyped command.
const MAX_LEN: usize = 32;

/// Edits allowed before two names are simply different words. Two covers a
/// transposition and a slip; three starts proposing `grep` for `help`.
const MAX_EDITS: usize = 2;

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

/// The closest candidates to `typed`: the best one, and a runner-up when it is
/// exactly as close.
///
/// Returning one name when two are equally near is a coin flip presented as
/// advice. Typing `io` is one edit from both `in` and `ip`, and picking `in`
/// because it appears earlier in a table tells the reader something the shell
/// does not actually know. Two names is the honest answer, and the reader
/// recognises theirs instantly.
///
/// A third would be noise, so it stops at two. Ties beyond the second keep the
/// earlier candidate, which makes the answer stable rather than dependent on
/// how the list happens to be ordered today.
pub fn nearest_two<'c>(
    typed: &[u8],
    candidates: impl Iterator<Item = &'c [u8]>,
) -> (Option<&'c [u8]>, Option<&'c [u8]>) {
    let mut best: Option<(usize, &[u8])> = None;
    let mut second: Option<(usize, &[u8])> = None;
    for c in candidates {
        let d = distance(typed, c);
        if d > MAX_EDITS {
            continue;
        }
        // A one-letter command is not a typo of another one-letter command,
        // and proposing one reads as noise.
        if typed.len() <= 2 && d > 1 {
            continue;
        }
        match best {
            None => best = Some((d, c)),
            Some((bd, _)) if d < bd => {
                second = best;
                best = Some((d, c));
            }
            // Only an exact tie with the best is worth offering as an
            // alternative. A worse match is not a second answer, it is a
            // longer list.
            Some((bd, _)) if d == bd && second.is_none() => second = Some((d, c)),
            _ => {}
        }
    }
    let alt = match (best, second) {
        (Some((bd, _)), Some((sd, c))) if sd == bd => Some(c),
        _ => None,
    };
    (best.map(|(_, c)| c), alt)
}

/// The single closest candidate, for callers that want one answer.
pub fn nearest<'c>(typed: &[u8], candidates: impl Iterator<Item = &'c [u8]>) -> Option<&'c [u8]> {
    nearest_two(typed, candidates).0
}
