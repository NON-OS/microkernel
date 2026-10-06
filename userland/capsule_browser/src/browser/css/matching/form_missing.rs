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

use super::cx::Cx;
use super::form_required::required;
use super::form_select::{options, selected};
use super::form_value::input_suffering;

/* Whether a candidate control suffers from a constraint its markup sets
 * (Some(true)), satisfies them all (Some(false)), or cannot be judged
 * here (None). */
pub(super) fn suffering(cx: &Cx, id: usize) -> Option<bool> {
    let n = cx.element(id)?;
    match n.tag.as_str() {
        "input" => input_suffering(cx, id, n),
        "textarea" => Some(
            required(n)
                && n.children
                    .iter()
                    .all(|&c| cx.dom.nodes.get(c).is_none_or(|t| t.text.is_empty())),
        ),
        "select" => Some(required(n) && select_missing(cx, id)),
        _ => Some(false),
    }
}

/* A required select is missing its value when nothing is selected, or
 * when the one selected option is its placeholder label option: the first
 * option, a direct child of a single-choice select of display size one,
 * with an empty value. */
fn select_missing(cx: &Cx, id: usize) -> bool {
    let opts = options(cx, id);
    let mut chosen = opts.iter().copied().filter(|&o| selected(cx, o));
    let Some(first) = chosen.next() else { return true };
    let s = &cx.dom.nodes[id];
    let size = s.attr("size").and_then(|v| v.trim().parse::<u32>().ok()).filter(|&v| v > 0);
    let single = s.attr("multiple").is_none() && size.is_none_or(|v| v == 1);
    let o = &cx.dom.nodes[first];
    let value = match o.attr("value") {
        Some(v) => v.is_empty(),
        None => {
            o.children.iter().all(|&c| cx.dom.nodes.get(c).is_none_or(|t| t.text.trim().is_empty()))
        }
    };
    single && opts.first() == Some(&first) && o.parent == id && value && chosen.next().is_none()
}
