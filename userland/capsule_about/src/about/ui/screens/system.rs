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
use nonos_toolkit::widgets::Rect;

use crate::about::state::State;

use super::super::metrics::metrics;
use super::{system_build, system_runtime, system_space, system_uptime};

// Two columns of fixed-height cards, so the extent is the taller of the two rather
// than a measurement. Build and Uptime are what the image is and how long it has
// been itself; Address space and Runtime are where it lives and what it is doing.
pub fn content_h(_rect: &Rect) -> u32 {
    let gap = metrics().card_gap;
    let left = system_build::height() + gap + system_uptime::height();
    left.max(system_space::height() + gap + system_runtime::height())
}

pub fn paint(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    let mut pane = fb.sub(rect.x, rect.y, rect.w, rect.h);
    let y = -(state.scroll as i32);
    let gap = metrics().card_gap;
    let col = (rect.w.saturating_sub(gap)) / 2;
    let right_x = col + gap;
    let right_w = rect.w.saturating_sub(right_x);
    system_build::paint(&mut pane, 0, y, col);
    system_uptime::paint(&mut pane, 0, y + (system_build::height() + gap) as i32, col);
    system_space::paint(&mut pane, right_x, y, right_w);
    let runtime_y = y + (system_space::height() + gap) as i32;
    system_runtime::paint(&mut pane, right_x, runtime_y, right_w);
}
