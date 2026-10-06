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

use alloc::vec::Vec;

use crate::browser::manifest::HEIGHT;

use super::media_value::{compare, feature, length, ratio};

/* Media Queries 4 range syntax for width, height and aspect-ratio: `name
 * op value`, `value op name`, and `value op name op value` with both
 * operators pointing the same way. None when the body is not in range form
 * (it has a colon, or no comparison), so the colon forms take it. A range
 * on any other feature, or a value that does not parse, fails closed. */
pub(super) fn range(body: &str, viewport_w: u32) -> Option<bool> {
    if body.contains(':') || !body.contains(['<', '>', '=']) {
        return None;
    }
    let (mut terms, mut ops): (Vec<&str>, Vec<&str>) = (Vec::new(), Vec::new());
    let mut rest = body;
    while let Some(at) = rest.find(['<', '>', '=']) {
        let len = if rest[at..].starts_with("<=") || rest[at..].starts_with(">=") { 2 } else { 1 };
        terms.push(rest[..at].trim());
        ops.push(&rest[at..at + len]);
        rest = &rest[at + len..];
    }
    terms.push(rest.trim());
    let (w, h) = (viewport_w as f32, HEIGHT as f32);
    let Some(at) = terms.iter().position(|t| feature(t, w, h).is_some()) else {
        return Some(false);
    };
    let is_ratio = terms[at] == "aspect-ratio";
    let val = |t: &str| feature(t, w, h).or_else(|| if is_ratio { ratio(t) } else { length(t) });
    let holds = |i: usize| match (val(terms[i]), val(terms[i + 1])) {
        (Some(x), Some(y)) => compare(x, ops[i], y),
        _ => false,
    };
    Some(match (terms.len(), at) {
        (2, _) => holds(0),
        (3, 1) if ops[0] != "=" && ops[0].as_bytes()[0] == ops[1].as_bytes()[0] => {
            holds(0) && holds(1)
        }
        _ => false,
    })
}
