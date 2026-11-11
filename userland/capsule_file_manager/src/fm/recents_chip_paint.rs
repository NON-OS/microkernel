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
use nonos_toolkit::widgets::{Pill, Rect, State as Look};

use super::chip::chip_h;
use super::filetype::Kind;
use super::recents_chips::KindChip;
use super::state::State;
use super::target::Target;
use super::target_at::look;

/// The chip covering `(x, y)`, by its index in `chips`; its `kind` is the
/// filter it selects.
pub fn chip_at(chips: &[KindChip], x: u32, y: u32) -> Option<usize> {
    chips.iter().position(|c| x >= c.x && x < c.x + c.w && y >= c.y && y < c.y + chip_h())
}

/// Draws the row from the same slots the hit-test walks, each chip a shared
/// `Pill`, the active one selected.
pub fn paint_chips(state: &State, fb: &mut PaintBuffer, chips: &[KindChip], active: Option<Kind>) {
    for (i, c) in chips.iter().enumerate() {
        let lit = Look { selected: c.kind == active, ..look(state, Target::Chip(i)) };
        Pill::new(c.label.as_str())
            .state(lit)
            .draw(fb, Rect { x: c.x, y: c.y, w: c.w, h: chip_h() });
    }
}
