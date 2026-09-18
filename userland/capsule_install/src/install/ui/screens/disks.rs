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

//! The disks, one row each: bus, size, and what is on it now. A driver
//! that did not answer gets a row too, in the fault colour, so the list
//! says everything the machine has rather than everything that worked.

use nonos_app_skeleton::PaintBuffer;

use super::disk_row::row;
use crate::install::state::State;
use crate::install::ui::frame::Body;
use crate::install::ui::metrics::{BODY_PX, RADIUS, ROW_H, SMALL_PX};
use crate::install::ui::text::top_of;
use crate::install::ui::{text, theme};

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    if state.disks.is_empty() {
        text::line(
            fb,
            b.x,
            b.y,
            "No block driver is serving a disk on this boot.",
            theme::DANGER,
            BODY_PX,
        );
        return;
    }
    let mut y = b.y;
    for (i, d) in state.disks.iter().enumerate() {
        let selected = i == state.selected;
        if selected {
            fb.fill_round(b.x, y, b.w, ROW_H, RADIUS, theme::SELECTED_BG);
            fb.fill_rect(b.x, y + 12, 3, ROW_H - 24, theme::ACCENT);
        }
        row(fb, d, b.x, y, b.w);
        y += ROW_H + 6;
    }
    let note =
        "The disk you choose is erased completely. The stick you booted from is not in this list.";
    text::line(fb, b.x, top_of(b.y + b.h - 30, 30, SMALL_PX), note, theme::MUTED, SMALL_PX);
}
