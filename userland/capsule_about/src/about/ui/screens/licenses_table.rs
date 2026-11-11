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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left, mono_left, truncate, truncate_mono};
use nonos_toolkit::widgets::{Rect, Separator};

use crate::about::data::third_party::COMPONENTS;

use super::super::card::{self, titled};
use super::super::kv::row_h;
use super::super::metrics::metrics;
use super::super::mono::valid;
use super::licenses_cols::{band, columns};

pub const HEAD_COMPONENT: &[u8] = b"COMPONENT";
pub const HEAD_ROLE: &[u8] = b"ROLE";
const HEAD_LICENSE: &[u8] = b"LICENCE";
const HEAD_RULE_GAP: u32 = 6;

pub fn height() -> u32 {
    card::overhead() + row_h() + scale().px(HEAD_RULE_GAP) + row_h() * COMPONENTS.len() as u32
}

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let (r, m, s) = (roles(), metrics(), scale());
    let (value, muted, pad, body, num) = (r.text_2, r.muted, m.card_pad, m.body_px, m.num_px);
    let (row, gap) = (row_h(), s.px(HEAD_RULE_GAP));
    let top = titled(fb, 0, y, w, height(), b"Third-party components");
    let (role_x, license_x) = columns();
    let inner = card::inner(w);
    let head = top + centre_top(0, row, body);
    left(fb, pad, head, valid(HEAD_COMPONENT), muted, body);
    left(fb, role_x, head, valid(HEAD_ROLE), muted, body);
    left(fb, license_x, head, valid(HEAD_LICENSE), muted, body);
    if let Ok(y) = u32::try_from(top + (row + gap / 2) as i32) {
        Separator::new().draw(fb, Rect { x: pad, y, w: inner, h: s.hair() });
    }
    let first = top + (row + gap) as i32;
    let name_w = role_x.saturating_sub(pad + m.kv_gap);
    let license_w = (pad + inner).saturating_sub(license_x);
    for (i, c) in COMPONENTS.iter().enumerate() {
        let row_y = first + (i as u32 * row) as i32;
        if i % 2 == 1 {
            band(fb, row_y, inner);
        }
        let cell = row_y + centre_top(0, row, body);
        left(fb, pad, cell, &truncate(valid(c.name), name_w, body), value, body);
        let role = truncate_mono(valid(c.role), license_x - role_x - m.kv_gap, num);
        mono_left(fb, role_x, cell, &role, muted, num);
        let license = truncate_mono(valid(c.license), license_w, num);
        mono_left(fb, license_x, cell, &license, value, num);
    }
}
