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

use crate::browser::css::WhiteSpace;

use super::super::abs_out_of_flow::out_of_flow;
use super::super::edges_x::edges_x;
use super::super::geom::margins::margins;
use super::super::tree::{BoxKind, BoxNode};
use super::inline_flow::Flow;
use super::inline_sink::{edges, line_end, Sink};
use super::inline_text::text_items;

const MAX_DEPTH: u32 = 400;

/* Walk an inline run in order: text through its white-space mode, atoms
 * whole, inline boxes by their children between their edges. A block
 * stranded inside an inline sits on lines of its own. */
pub(in super::super) fn walk(
    children: &[BoxNode],
    flow: &mut Flow,
    sink: &mut dyn Sink,
    depth: u32,
) {
    if depth > MAX_DEPTH {
        return;
    }
    for c in children.iter().filter(|c| !out_of_flow(&c.style)) {
        match &c.kind {
            BoxKind::Text(t) if t == "\n" => line_end(flow, sink),
            BoxKind::Text(t) => text_items(c, t, flow, sink),
            BoxKind::Image { .. } | BoxKind::InlineBlock => {
                let lead = flow.lead(true, c.style.white_space != WhiteSpace::Nowrap);
                sink.atom(c, lead, depth + 1);
            }
            BoxKind::Inline => {
                let ([_, mr, _, ml], (el, er)) = (margins(&c.style, 0), edges_x(&c.style));
                edges(c, [(ml, 0), (el, c.style.bg)], flow, sink);
                walk(&c.children, flow, sink, depth + 1);
                edges(c, [(er, c.style.bg), (mr, 0)], flow, sink);
            }
            BoxKind::Block | BoxKind::Flex | BoxKind::Grid => {
                if !flow.at_start {
                    line_end(flow, sink);
                }
                walk(&c.children, flow, sink, depth + 1);
                if !flow.at_start {
                    line_end(flow, sink);
                }
            }
        }
    }
}
