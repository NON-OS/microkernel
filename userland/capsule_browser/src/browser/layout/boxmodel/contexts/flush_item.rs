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

use super::super::ctx::Ctx;
use super::super::display_list::{Content, DisplayList, Fragment};
use super::super::inline_items::InlineItem;

/* Emit one line item with its slot at `r` ([x, line top, line height]).
 * `ul_end` is where the underlined word before it ended, if one did on
 * this line; the return value is the same for this item. */
pub(in super::super) fn flush_item(
    frags: &mut DisplayList,
    item: InlineItem,
    r: [i32; 3],
    ul_end: Option<i32>,
    ctx: &Ctx,
) -> Option<i32> {
    let [x, top, line_h] = r;
    match item {
        InlineItem::Word { text, ink, href, adv, node, lead, .. } => {
            /* An unpainted edge, or preserved white space with nothing to
             * paint under it, is only room on the line. */
            if ink.bg == 0
                && (text.is_empty() || (!ink.underline && text.bytes().all(|b| b == b' ')))
            {
                return None;
            }
            let joined =
                ink.underline && lead.ul && lead.space > 0 && ul_end == Some(x - lead.space);
            let (x, w, text) = match joined {
                true => (x - lead.space, adv + lead.space, [" ", &text].concat()),
                false => (x, adv, text),
            };
            let (px, color, bold, mono, font) =
                (ink.px as f32, ink.color, ink.bold, ink.mono, ink.font);
            let (underline, spacing, italic) = (ink.underline, ink.spacing, ink.italic);
            let text =
                Content::Text { text, color, px, bold, mono, underline, font, spacing, italic };
            frags.push(Fragment::leaf([x, top, w, line_h], ink.bg, text, href, node, ctx));
            underline.then_some(x + w)
        }
        InlineItem::Image { src, alt, w, h, href, node, fit, .. } => {
            let r = [x, top + (line_h - h).max(0) / 2, w, h];
            frags.push(Fragment::leaf(r, 0, Content::Image { src, alt, fit }, href, node, ctx));
            None
        }
        InlineItem::Atom { frags: sub, h, .. } => {
            /* The atom was laid out at the origin; it moves into its slot
             * sitting on the line's bottom edge, as its text would. */
            let (dx, dy) = (x, top + (line_h - h).max(0));
            for mut f in sub {
                (f.x, f.y) = (f.x + dx, f.y + dy);
                if let Some(c) = f.clip.as_mut() {
                    *c = [c[0] + dx, c[1] + dy, c[2] + dx, c[3] + dy];
                }
                frags.push(f);
            }
            None
        }
        InlineItem::Break => None,
    }
}
