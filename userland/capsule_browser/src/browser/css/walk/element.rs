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

use alloc::boxed::Box;
use alloc::rc::Rc;

use crate::browser::css::apply_style_attr::style_attr;
use crate::browser::css::computed::Computed;
use crate::browser::css::vars::{At, VarScope};

use super::custom::custom_scope;
use super::order::Order;
use super::pres_hints::pres_hints;
use super::styling::Styling;
use super::walker::Walker;

/* The computed style of element `id` under `parent`, into `out`, and the
 * custom property scope its children inherit. Its background image and grid
 * placement go to the walk's output. Kept out of line: its locals would
 * otherwise sit in every frame of the recursive walk. */
#[inline(never)]
pub(in crate::browser::css) fn style(
    w: &mut Walker,
    id: usize,
    parent: &Computed,
    scope: &Rc<VarScope>,
    out: &mut Computed,
) -> Rc<VarScope> {
    w.match_ua(id);
    w.match_author(id, false);
    let dom = w.dom;
    let node = &dom.nodes[id];
    let inline = style_attr(node);
    let hints = pres_hints(dom, id, w.link.as_deref());
    let order = Order {
        ua: (w.ua.rules, &w.ua_hits),
        hints: &hints,
        author: (w.author.rules, &w.author_hits),
        inline: &inline,
    };
    let scope = custom_scope(&order, scope, w.props, id);
    let mut st = Styling::new(parent, At { scope: &scope, props: w.props, id });
    super::table_start::table_start(&mut st.c, parent, &node.tag, dom.quirks);
    st.run(&order);
    let Styling { mut c, bg, grid, counters, vars, .. } = st;
    w.out.svg_paint[id] = super::svg_paint::svg_paint(node, &order, vars, &c);
    if node.tag == "noscript" && !w.noscript {
        c.display_none = true;
    }
    super::spans::spans(&mut c, node);
    w.out.bg_images[id] = bg;
    w.out.grids[id] = grid.map(Box::new);
    if let Some(k) = w.counters.as_mut() {
        k.element(node, &counters, node.parent);
    }
    *out = c;
    scope
}
