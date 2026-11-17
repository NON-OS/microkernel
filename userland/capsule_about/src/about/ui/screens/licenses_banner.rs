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
use nonos_toolkit::widgets::text::left;

use crate::about::data::license::{NAME, SPDX, URL, VERSION};

use super::super::card;
use super::super::kv::{kv, row_h};
use super::super::metrics::metrics;
use super::super::mono::valid;

const NAME_H: u32 = 30;
const NAME_GAP: u32 = 10;

fn name_step() -> u32 {
    let s = scale();
    s.px(NAME_H) + s.px(NAME_GAP)
}

pub fn height() -> u32 {
    metrics().card_pad * 2 + name_step() + row_h() * 3
}

// The banner states the licence this image ships under before the text of it, so
// the identifier a reader actually needs is not buried in the scroll body.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let (r, m) = (roles(), metrics());
    let top = card::panel(fb, 0, y, w, height());
    left(fb, m.card_pad, top, valid(NAME), r.text, m.title_px);
    let rows: [(&[u8], &[u8], bool); 3] =
        [(b"Version", VERSION, false), (b"SPDX", SPDX, true), (b"Text", URL, false)];
    let (first, step) = (top + name_step() as i32, row_h());
    for (i, (label, value, num)) in rows.into_iter().enumerate() {
        kv(fb, m.card_pad, first + (i as u32 * step) as i32, card::inner(w), label, value, num);
    }
}
