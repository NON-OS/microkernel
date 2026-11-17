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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw as paint, text, Card, Rect, Separator};

use super::crumbs::{crumb_parts, crumb_slots};
use super::header_layout::tool_y;
use super::header_slots::{strip, HeadHit, CRUMB_PAD, CRUMB_PX, CRUMB_SEP_W, HOME_W};
use super::layout::{font, hair, metrics, px};
use super::paint_toolbar::paint_toolbar;
use super::state::State;
use super::target::Target;
use super::toolbar_buttons::{glyph_y, plain, plain_icon};

// The separator chevron is drawn a size down from a control glyph so it reads as
// punctuation between segments rather than as another button.
const SEP_S: u32 = 12;

pub fn paint_header(state: &State, fb: &mut PaintBuffer) {
    let (w, m, line) = (fb.width, metrics(), hair());
    let (left, head) = (m.sidebar_w, m.header_h);
    let cw = w.saturating_sub(left);
    paint::fill(fb, Rect { x: left, y: 0, w: cw, h: head }, roles().sunken);
    Separator::new().draw(fb, Rect { x: left, y: head - line, w: cw, h: line });
    paint_crumbs(state, fb);
    paint_toolbar(state, fb);
}

// The capsule and its separators are derived from the very slots the segments
// were laid out into, so the measured layout stays the only source of geometry.
// Each segment is a `Plain` button body, so it shows hover and press.
fn paint_crumbs(state: &State, fb: &mut PaintBuffer) {
    let slots = crumb_slots(state);
    let (Some(first), Some(last)) = (slots.first(), slots.last()) else { return };
    let parts = crumb_parts(state);
    let y = tool_y();
    let (pad, sep_w, sep_s, crumb_px, tool_h) =
        (px(CRUMB_PAD), px(CRUMB_SEP_W), px(SEP_S), font(CRUMB_PX), strip().h);
    let (x, r) = (first.x.saturating_sub(pad), roles());
    let w = (last.x + last.w + pad).saturating_sub(x);
    Card::new().draw(fb, Rect { x, y, w, h: tool_h });
    let tail = slots.len() - 1;
    for (n, slot) in slots.iter().enumerate() {
        let HeadHit::Crumb(i) = slot.hit else { continue };
        if n > 0 {
            let cx = slot.x.saturating_sub(sep_w - (sep_w - sep_s) / 2);
            paint::icon(fb, IconId::ChevronRight, cx, glyph_y(y, sep_s), sep_s, r.muted);
        }
        if i == 0 {
            plain_icon(state, fb, slot, y, (IconId::Home, HOME_W));
            continue;
        }
        plain(state, fb, slot, y, Target::Head(slot.hit));
        let Some(part) = parts.get(i) else { continue };
        let ink = if n == tail { r.text } else { r.muted };
        text::left(fb, slot.x, text::centre_top(y, tool_h, crumb_px), part, ink, crumb_px);
    }
}
