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

use alloc::string::String;

use super::super::inline_items::Lead;
use super::super::tree::BoxNode;
use super::inline_flow::Flow;

/* Where an inline walk delivers what it finds. Line layout builds items
 * from it and the intrinsic measure sums widths, so both see the same
 * words, spaces and break opportunities. */
pub(in super::super) trait Sink {
    /* A word of text box `c`, transformed and measured. */
    fn word(&mut self, c: &BoxNode, text: String, adv: i32, lead: Lead);
    /* An atomic inline: an inline-block, inline flex or grid, image or
     * form control. */
    fn atom(&mut self, c: &BoxNode, lead: Lead, depth: u32);
    /* `w` px of an inline box's own edge, painted in `bg`. */
    fn edge(&mut self, c: &BoxNode, w: i32, bg: u32, lead: Lead);
    fn hard_break(&mut self);
}

/* A forced line end, seen by both the sink and the white-space state. */
pub(in super::super) fn line_end(flow: &mut Flow, sink: &mut dyn Sink) {
    sink.hard_break();
    flow.hard_break();
}

/* An inline box's margin (unpainted) and its padding and border (in its
 * background), in the order the side meets them. */
pub(in super::super) fn edges(
    c: &BoxNode,
    parts: [(i32, u32); 2],
    flow: &mut Flow,
    sink: &mut dyn Sink,
) {
    for (w, bg) in parts.into_iter().filter(|&(w, _)| w > 0) {
        let lead = flow.lead(false, false);
        sink.edge(c, w, bg, lead);
    }
}
