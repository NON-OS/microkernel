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

/* The footer: the keys this screen takes, from `hints`, one set on each side. */

use nonos_app_skeleton::PaintBuffer;

use super::hints::hints;
use super::metrics::{FOOTER_H, PAD, SMALL_PX};
use super::text::{right, top_of};
use super::{text, theme};
use crate::install::state::{Screen, State};

pub fn paint(fb: &mut PaintBuffer, state: &State, w: u32, h: u32) {
    let y = h - FOOTER_H;
    fb.fill_rect(0, y, w, 1, theme::RULE);
    let top = top_of(y, FOOTER_H, SMALL_PX);
    let (left, right_hint) = hints(state);
    text::line(fb, PAD, top, left, theme::MUTED, SMALL_PX);
    let colour = if matches!(state.screen, Screen::Writing | Screen::Verifying) {
        theme::WARN
    } else {
        theme::FOREGROUND
    };
    right(fb, w - PAD, top, right_hint, colour, SMALL_PX);
}
