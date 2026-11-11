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
use super::header_slots::{HeadHit, CRUMB_PAD, CRUMB_PX, CRUMB_SEP_W, HOME_W, TOOL_H};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::layout::{CONTENT_X, HEADER_H};
use super::paint_toolbar::paint_toolbar;
use super::state::State;
use super::target::Target;
use super::toolbar_buttons::{glyph_y, plain};

// The separator chevron is drawn a size down from a control glyph so it reads as
// punctuation between segments rather than as another button.
const SEP_S: u32 = 12;

pub fn paint_header(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width;
    let cw = w.saturating_sub(CONTENT_X);
    paint::fill(fb, Rect { x: CONTENT_X, y: 0, w: cw, h: HEADER_H }, roles().sunken);
    Separator::new().draw(fb, Rect { x: CONTENT_X, y: HEADER_H - 1, w: cw, h: 1 });
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
    let (x, r) = (first.x.saturating_sub(CRUMB_PAD), roles());
    let w = (last.x + last.w + CRUMB_PAD).saturating_sub(x);
    Card::new().draw(fb, Rect { x, y, w, h: TOOL_H });
    let tail = slots.len() - 1;
    for (n, slot) in slots.iter().enumerate() {
        let HeadHit::Crumb(i) = slot.hit else { continue };
        plain(state, fb, slot, y, Target::Head(slot.hit));
        if n > 0 {
            let cx = slot.x.saturating_sub(CRUMB_SEP_W - (CRUMB_SEP_W - SEP_S) / 2);
            paint::icon(fb, IconId::ChevronRight, cx, glyph_y(y, SEP_S), SEP_S, r.muted);
        }
        if i == 0 {
            draw(fb, Icon::Home, slot.x, glyph_y(y, HOME_W), HOME_W, r.muted.argb());
            continue;
        }
        let Some(part) = parts.get(i) else { continue };
        let ink = if n == tail { r.text } else { r.muted };
        text::left(fb, slot.x, text::centre_top(y, TOOL_H, CRUMB_PX), part, ink, CRUMB_PX);
    }
}
