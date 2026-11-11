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

use crate::clock::says::NOT_SET;
use crate::clock::state::State;
use crate::clock::tabs::Tab;
use crate::clock::{fmt, paint_analog, paint_settime, paint_stopwatch, paint_tabbar, paint_timer};

/// Where the clock tab's time, date lines and face sit, in logical pixels;
/// glyph sizes are multiples of the 8 pixel bitmap face at 1x.
const TIME_X: u32 = 36;
const TIME_Y: u32 = 72;
const TIME_GLYPHS: u32 = 4;
const LINE_X: u32 = 40;
const DAY_Y: u32 = 122;
const DATE_Y: u32 = 142;
const DAY_OF_MONTH_X: u32 = 84;
const YEAR_X: u32 = 120;
const LINE_GLYPHS: u32 = 1;
const FACE_X: u32 = 180;
const FACE_Y: u32 = 300;
const FACE_RADIUS: u32 = 84;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(roles().bg.argb());
    match state.tab {
        Tab::Clock => paint_clock(state, fb),
        Tab::Stopwatch => paint_stopwatch::paint(state, fb),
        Tab::Timer => paint_timer::paint(state, fb),
        Tab::Set => paint_settime::paint(state, fb),
    }
    paint_tabbar::paint(state, fb);
}

fn paint_clock(state: &State, fb: &mut PaintBuffer) {
    let (theme, s) = (roles(), scale());
    let line = s.px(LINE_GLYPHS);
    /* Drawn from no reading the face would show midnight of year zero. */
    if !state.clock_ok {
        fb.text_scaled(s.px(LINE_X), s.px(DAY_Y), NOT_SET, theme.danger.argb(), line);
        return;
    }
    let (r, muted) = (&state.rtc, theme.muted.argb());
    let hms = fmt::hms(r.hour, r.minute, r.second);
    fb.text_scaled(s.px(TIME_X), s.px(TIME_Y), &hms, theme.text.argb(), s.px(TIME_GLYPHS));
    let wd = fmt::weekday(r.year, r.month, r.day);
    fb.text_scaled(s.px(LINE_X), s.px(DAY_Y), wd.as_bytes(), muted, line);
    let mon = fmt::month_name(r.month);
    fb.text_scaled(s.px(LINE_X), s.px(DATE_Y), mon.as_bytes(), muted, line);
    let dd = fmt::two(r.day);
    fb.text_scaled(s.px(DAY_OF_MONTH_X), s.px(DATE_Y), &dd, muted, line);
    let yr = fmt::year4(r.year);
    fb.text_scaled(s.px(YEAR_X), s.px(DATE_Y), &yr, muted, line);
    let (cx, cy, radius) = (s.px(FACE_X), s.px(FACE_Y), s.px(FACE_RADIUS));
    paint_analog::paint(state, fb, cx as i32, cy as i32, radius as i32);
}
