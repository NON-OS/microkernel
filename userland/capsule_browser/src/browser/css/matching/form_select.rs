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

use super::cx::Cx;
use super::form_disabled::disabled;

/* A select's list of options in tree order: its option children and the
 * options of its optgroup children (HTML 4.10.7). */
pub(super) fn options(cx: &Cx, select: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let Some(s) = cx.element(select) else { return out };
    for &c in &s.children {
        match cx.element(c).map(|n| n.tag.as_str()) {
            Some("option") => out.push(c),
            Some("optgroup") => out.extend(
                cx.dom.nodes[c]
                    .children
                    .iter()
                    .copied()
                    .filter(|&o| cx.element(o).is_some_and(|n| n.tag == "option")),
            ),
            _ => {}
        }
    }
    out
}

/* The select an option belongs to: its parent, or its optgroup's. */
fn owning_select(cx: &Cx, id: usize) -> Option<usize> {
    let p = cx.element(cx.element(id)?.parent)?;
    let at = if p.tag == "optgroup" { p.parent } else { cx.element(id)?.parent };
    cx.element(at).filter(|s| s.tag == "select").map(|_| at)
}

/* Whether an option is selected (HTML's selectedness setting algorithm).
 * In a single-choice select the last option carrying `selected` wins; with
 * none carrying it and a display size of one, the first option that is not
 * disabled is selected. A multiple select selects what carries the
 * attribute. */
pub(super) fn selected(cx: &Cx, id: usize) -> bool {
    let own = cx.element(id).is_some_and(|n| n.attr("selected").is_some());
    let Some(sel) = owning_select(cx, id) else { return own };
    let s = &cx.dom.nodes[sel];
    if s.attr("multiple").is_some() {
        return own;
    }
    let opts = options(cx, sel);
    if let Some(&last) = opts.iter().rev().find(|&&o| cx.dom.nodes[o].attr("selected").is_some()) {
        return last == id;
    }
    let size = s.attr("size").and_then(|v| v.trim().parse::<u32>().ok()).filter(|&v| v > 0);
    size.is_none_or(|v| v == 1) && opts.iter().find(|&&o| !disabled(cx.dom, o)) == Some(&id)
}
