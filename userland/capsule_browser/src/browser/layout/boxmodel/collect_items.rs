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
use alloc::vec::Vec;

use super::contexts::collect_atom::{atom, measured};
use super::contexts::inline_flow::Flow;
use super::contexts::inline_sink::Sink;
use super::contexts::inline_walk::walk;
use super::ctx::Ctx;
use super::inline_items::{Ink, InlineItem, Lead};
use super::tree::BoxNode;

/* Flatten an inline run into measured words, images, inline-block atoms
 * and hard breaks, each with the space and break opportunity before it,
 * for line layout `content_w` px wide, or None to measure it unwrapped. */
pub(super) fn collect_items(
    children: &[BoxNode],
    content_w: Option<i32>,
    out: &mut Vec<InlineItem>,
    depth: u32,
    ctx: Ctx,
) {
    let mut sink = Items { out, content_w, ctx };
    walk(children, &mut Flow::new(), &mut sink, depth);
}

struct Items<'a> {
    out: &'a mut Vec<InlineItem>,
    content_w: Option<i32>,
    ctx: Ctx,
}

impl Sink for Items<'_> {
    fn word(&mut self, c: &BoxNode, text: String, adv: i32, lead: Lead) {
        let (s, href, node) = (&c.style, c.href.clone(), c.dom_id);
        let (ink, line_h) = (Ink::of(s, s.bg, s.underline), s.line_height() as i32);
        self.out.push(InlineItem::Word { text, ink, href, adv, line_h, node, lead });
    }

    fn atom(&mut self, c: &BoxNode, lead: Lead, depth: u32) {
        let it = match self.content_w {
            Some(w) => atom(c, w, lead, depth, self.ctx),
            None => measured(c, lead, depth),
        };
        self.out.push(it);
    }

    /* An edge is a word with no text: it takes room and paints its box's
     * background, never an underline. */
    fn edge(&mut self, c: &BoxNode, w: i32, bg: u32, lead: Lead) {
        let (ink, line_h) = (Ink::of(&c.style, bg, false), c.style.line_height() as i32);
        let (href, node, text) = (c.href.clone(), c.dom_id, String::new());
        self.out.push(InlineItem::Word { text, ink, href, adv: w, line_h, node, lead });
    }

    fn hard_break(&mut self) {
        self.out.push(InlineItem::Break);
    }
}
