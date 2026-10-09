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

use alloc::rc::Rc;

use crate::browser::css::computed::Computed;
use crate::browser::css::pseudo_style::{after, before};
use crate::browser::css::vars::VarScope;
use crate::browser::dom::node::NodeKind;

use super::element;
use super::walker::Walker;

/* Deeper subtrees keep their ancestor's style: it bounds the recursion. */
const MAX_DEPTH: u32 = 400;

/* The parent's style, node id and custom-property scope. */
type Up<'a> = (&'a Computed, usize, &'a Rc<VarScope>);

/* Cascade node `id` and its subtree under `up`. An element's ::before and
 * other pseudo-elements cascade before its children (after its counters
 * step), its ::after once they are done, so counters and quotes read in
 * document order. A text node owns no style: it reads its parent's. The
 * frame holds one style: it recurses once per level of the tree. */
pub(in crate::browser::css) fn walk(w: &mut Walker, id: usize, up: Up, depth: u32) {
    let (dom, (parent, parent_id, scope)) = (w.dom, up);
    let Some(node) = dom.nodes.get(id) else { return };
    let mut c = Computed::root();
    let (scope, pending) = match node.kind {
        NodeKind::Element => {
            let sc = element::style(w, id, parent, scope, &mut c);
            let pending = before(w, id, &c, &sc);
            (sc, pending)
        }
        NodeKind::Document => {
            c = Computed::inherit_from(parent);
            (scope.clone(), None)
        }
        _ => {
            w.out.styles.share(id, parent_id);
            return;
        }
    };
    w.out.styles.set(id, &c);
    /* The root element's font size is what rem resolves against. */
    if depth == 0 || (depth == 1 && node.kind == NodeKind::Element) {
        crate::browser::css::calc::viewport::set_root_font(c.font_px);
    }
    if depth < MAX_DEPTH {
        for &ch in &node.children {
            walk(w, ch, (&c, id, &scope), depth + 1);
        }
    }
    if let Some(hits) = pending {
        after(w, id, &c, &scope, hits);
    }
    if let Some(k) = w.counters.as_mut() {
        k.leave(id);
    }
}
