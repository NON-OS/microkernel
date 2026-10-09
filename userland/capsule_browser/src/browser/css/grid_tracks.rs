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

use crate::browser::css::computed::GridTrack;

use super::matching_paren::matching_paren;
use super::one_track::one_track;

/* A grid-template-columns or grid-template-rows track list: lengths,
 * percentages, fractions, the content keywords, minmax() and repeat(n, ...)
 * of one or more tracks. Tracks past the cap of N drop; a list with no
 * track at all is rejected. A repeat() inside a repeat() is not CSS and is
 * skipped, which also bounds the recursion to one level. */
pub(super) fn parse_grid_tracks<const N: usize>(
    value: &str,
    em: u32,
) -> Option<([GridTrack; N], u8)> {
    let (mut out, mut n, mut rest) = ([GridTrack::Auto; N], 0usize, value.trim());
    while !rest.is_empty() && n < N {
        let (tok, next) = next_token(rest);
        rest = next;
        let mut push = |t: GridTrack| {
            if n < N {
                (out[n], n) = (t, n + 1);
            }
        };
        if starts(tok, "repeat(") {
            let body = tok.get(7..tok.len().saturating_sub(1)).unwrap_or("");
            let (count, list) = body.split_once(',').unwrap_or(("0", ""));
            let count = count.trim().parse::<usize>().unwrap_or(0).min(N);
            let nested = list.as_bytes().windows(7).any(|w| w.eq_ignore_ascii_case(b"repeat("));
            if let Some((inner, k)) = (!nested).then(|| parse_grid_tracks::<N>(list, em)).flatten()
            {
                let k = k as usize;
                inner.iter().take(k).cycle().take(count * k).for_each(|t| push(*t));
            }
        } else if let Some(track) = one_track(tok, em) {
            push(track);
        }
    }
    (n > 0).then_some((out, n as u8))
}

fn starts(s: &str, p: &str) -> bool {
    s.len() >= p.len() && s.as_bytes()[..p.len()].eq_ignore_ascii_case(p.as_bytes())
}

/* The next track token and the text after it. A [named line] group is
 * skipped (grid_lines records it); a function runs to its close paren. */
fn next_token(rest: &str) -> (&str, &str) {
    let rest = rest.trim_start();
    if let Some(after) = rest.strip_prefix('[') {
        let close = after.find(']').map_or(after.len(), |i| i + 1);
        return ("", after.get(close..).unwrap_or(""));
    }
    let func = starts(rest, "repeat(") || starts(rest, "minmax(") || starts(rest, "fit-content(");
    let end = match (func, rest.find('(')) {
        (true, Some(o)) => (o + 1 + matching_paren(&rest[o + 1..]) + 1).min(rest.len()),
        _ => rest.find(char::is_whitespace).unwrap_or(rest.len()),
    };
    (&rest[..end], &rest[end..])
}
