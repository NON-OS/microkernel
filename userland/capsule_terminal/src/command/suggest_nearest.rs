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

//! The one or two names nearest to what was typed.

use super::{distance, MAX_EDITS};

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
