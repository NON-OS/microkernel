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

use crate::browser::css::ua::noscript_visible;
use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

/* Whether this cascade renders <noscript> content, given whether QuickJS
 * is on and whether the page's scripts ran without failing, when known
 * (`js`). */
pub(in crate::browser::css) fn shows(dom: &Dom, js: (bool, Option<bool>)) -> bool {
    let outside = js.0 && js.1 != Some(false) && text_outside(dom);
    noscript_visible(js.0, js.1, outside)
}

/* Whether <body> holds text outside <noscript> (and outside script, style
 * and template, which never render). A page with no <noscript> at all
 * answers yes: it has nothing to show in its place. */
fn text_outside(dom: &Dom) -> bool {
    if !dom.nodes.iter().any(|n| n.tag == "noscript") {
        return true;
    }
    let skip = |t: &str| matches!(t, "noscript" | "script" | "style" | "template");
    let body = dom.nodes.iter().position(|n| n.kind == NodeKind::Element && n.tag == "body");
    let mut stack: Vec<usize> = body.into_iter().collect();
    while let Some(id) = stack.pop() {
        let Some(n) = dom.nodes.get(id) else { continue };
        match n.kind {
            NodeKind::Text if !n.text.trim().is_empty() => return true,
            NodeKind::Element if !skip(&n.tag) => stack.extend(n.children.iter().copied()),
            _ => {}
        }
    }
    false
}
