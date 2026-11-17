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

//! One class of authority, as one row.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{mono_right, mono_width};
use nonos_toolkit::widgets::Tone;

use crate::pm::format::u32_decimal;

use super::super::metrics::{card_metrics, metrics};
use super::super::text;
use super::ovw_auth_delta::delta;
use super::ovw_authority::PAD;

// The swatch is the strip's slot, so the eye that learned the four positions in
// the table reads this card without relearning them.
const KEY_H: u32 = 12;

// One class: its colour key, its name, the change since the session started, and
// the count itself hard against the right edge so four rows align on their last
// digit. A class nobody holds keeps its row and dims instead of vanishing, since
// an empty row is a fact and a missing one is a gap in the reader's knowledge.
pub(super) fn row(
    fb: &mut PaintBuffer,
    at: (u32, u32, u32),
    h: u32,
    label: &[u8],
    counts: (u32, u32),
    tone: Tone,
) {
    let (r, (x, y, w)) = (roles(), at);
    let (held, was) = counts;
    let (fonts, c, s) = (metrics(), card_metrics(), scale());
    let (pad, key_h, px, body_px) = (s.px(PAD), s.px(KEY_H), fonts.num_px, fonts.body_px);
    let tint = if held == 0 { r.muted } else { tone.colour(&r) };

    let key_y = y + h.saturating_sub(key_h) / 2;
    let key = if held == 0 { r.border_strong } else { tone.colour(&r) };
    fb.fill_rect(x + pad, key_y, c.risk_slot_w, key_h, key.argb());

    let top = text::centred_top(y, h, body_px);
    let text_x = x + pad + c.risk_slot_w + c.risk_slot_gap + c.card_label_gap;
    text::left(fb, text_x, top, label, tint, body_px);

    let mut buf = [0u8; 12];
    let n = u32_decimal(held, &mut buf);
    let right = x + w.saturating_sub(pad);
    mono_right(fb, right, top as i32, text::valid(&buf[..n]), tint, px);

    /*
     * The delta is drawn only when there is one. A column of "+0" would be four
     * more numbers to read past on a card whose whole point is that a change is
     * rare enough to be worth noticing.
     */
    if held != was {
        let count_w = mono_width(text::valid(&buf[..n]), px);
        let delta_right = right.saturating_sub(count_w + c.card_label_gap);
        let mut mark = [0u8; 12];
        let m = delta(held, was, &mut mark);
        let tone = if held > was { r.warn } else { r.ok };
        mono_right(fb, delta_right, top as i32, text::valid(&mark[..m]), tone, px);
    }
}
