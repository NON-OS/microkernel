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

use super::crumbs::{crumb_parts, crumb_slots};
use super::header_slots::{HeadHit, CRUMB_PX, CRUMB_SEP, CRUMB_Y};
use super::layout::{CONTENT_X, HEADER_H};
use super::paint_toolbar::paint_toolbar;
use super::state::State;
use super::theme::{DEEP, INK, INK3, LINE};

pub fn paint_header(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width;
    let cw = w.saturating_sub(CONTENT_X);
    fb.fill_rect(CONTENT_X, 0, cw, HEADER_H, DEEP);
    fb.fill_rect(CONTENT_X, HEADER_H - 1, cw, 1, LINE);
    paint_crumbs(state, fb);
    paint_toolbar(state, fb);
}

// Separators are painted into the gap `crumb_slots` already reserved for them,
// so the measured layout stays the only source of horizontal positions.
fn paint_crumbs(state: &State, fb: &mut PaintBuffer) {
    let parts = crumb_parts(state);
    let mut prev_end = 0u32;
    for slot in crumb_slots(state) {
        let HeadHit::Crumb(i) = slot.hit else { continue };
        let Some(text) = parts.get(i) else { continue };
        if i > 0 {
            let _ = fb.text_ttf(prev_end as i32, CRUMB_Y as i32, CRUMB_SEP, INK3, CRUMB_PX);
        }
        let _ = fb.text_ttf(slot.x as i32, CRUMB_Y as i32, text, INK, CRUMB_PX);
        prev_end = slot.x + slot.w;
    }
}
