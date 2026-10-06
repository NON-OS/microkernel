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

/*
 * A full-screen frame: the brand panel, then on the right a marker with the
 * step, the title, one line saying what the screen is for, the screen's own
 * body, and the keys under a full rule, each where the canvas's layout puts
 * it, at the canvas's scale.
 */

use alloc::format;

use nonos_app_skeleton::PaintBuffer;

use super::chrome::panel;
use super::layout::layout;
use super::subtitle::subtitle;
use crate::install::state::{Screen, State};
use crate::install::ui::footer::keys;
use crate::install::ui::frame::{screen, Body};
use crate::install::ui::hints::hints;
use crate::install::ui::text::{keys_px, line, title};
use nonos_brand::marker;
use nonos_brand::palette::{CYAN, GROUND, RULE, TEXT, TEXT_2, TEXT_3, WARN};

pub fn paint(state: &mut State, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    let l = layout(w, h);
    fb.clear(GROUND);
    panel(fb, &l, state.screen);
    let x = l.col_x;
    let step = format!("STEP {:02} OF 05", state.screen.step());
    marker(fb, x, l.marker_y, &step, CYAN, TEXT_3, l.caption_px);
    title(fb, x, l.title_y, state.screen.title(), TEXT, l.title_px);
    line(fb, x, l.sub_y, subtitle(state.screen, &state.boot), TEXT_2, l.body_px);
    screen(state, fb, Body { x, y: l.body_y, w: l.col_w, h: l.body_h, m: l.m });
    let hair = l.scale.px(1).max(1);
    fb.fill_rect(l.panel_w + 1, l.foot_y, w, hair, RULE);
    let (left, right) = hints(state);
    let busy = matches!(state.screen, Screen::Writing | Screen::Verifying);
    let edge = x + l.col_w;
    let px = keys_px(left, right, l.col_w, 2 * l.unit, l.caption_px, l.label_px);
    keys(fb, x, l.keys_y, left, TEXT_3, edge, px);
    keys(fb, 0, l.keys_y, right, if busy { WARN } else { TEXT_2 }, edge, px);
}
