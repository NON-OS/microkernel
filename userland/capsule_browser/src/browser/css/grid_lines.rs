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

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::matching_paren::matching_paren;
use super::one_track::one_track;

/* Named column lines from a grid-template-columns value: each [a b] group
 * names the line before the next track. Tokenization mirrors
 * parse_grid_tracks so the recorded indices line up with the parsed
 * tracks. repeat() expands to several tracks, so names after one would
 * drift: the rest stays unnamed rather than recorded at bad indices. */
pub(super) fn col_line_names(value: &str, em: u32) -> Vec<(String, u8)> {
    let (mut out, mut track): (Vec<(String, u8)>, u8) = (Vec::new(), 0);
    let mut rest = value.trim();
    let starts = |r: &str, p: &str| r.get(..p.len()).is_some_and(|h| h.eq_ignore_ascii_case(p));
    while !rest.is_empty() && out.len() < 32 {
        rest = rest.trim_start();
        if let Some(after) = rest.strip_prefix('[') {
            let close = after.find(']').unwrap_or(after.len());
            for name in after[..close].split_whitespace() {
                out.push((name.to_string(), track));
            }
            rest = after.get(close + 1..).unwrap_or("");
            continue;
        }
        if starts(rest, "repeat(") {
            break;
        }
        let end = if starts(rest, "minmax(") {
            (7 + matching_paren(&rest[7..]) + 1).min(rest.len())
        } else {
            rest.find(char::is_whitespace).unwrap_or(rest.len())
        };
        if one_track(&rest[..end], em).is_some() {
            track = track.saturating_add(1);
        }
        rest = rest.get(end..).unwrap_or("");
    }
    out
}

/* The quoted rows of a grid-template-areas value, each split into cell
 * tokens. Rows and cells are capped so a hostile sheet cannot balloon it. */
pub(super) fn area_rows(value: &str) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut rest = value;
    while rows.len() < 16 {
        let Some(open) = rest.find(['"', '\'']) else { break };
        let q = rest.as_bytes()[open] as char;
        let Some(len) = rest[open + 1..].find(q) else { break };
        let row = &rest[open + 1..open + 1 + len];
        let cells: Vec<String> = row.split_whitespace().take(32).map(|c| c.to_string()).collect();
        if !cells.is_empty() {
            rows.push(cells);
        }
        rest = &rest[open + len + 2..];
    }
    rows
}
