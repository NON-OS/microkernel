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

//! The progress bar: a track and a fill, no animation. What it shows is
//! bytes that have actually reached the driver, so it never runs ahead of
//! the disk and never has to walk back.

use nonos_app_skeleton::PaintBuffer;

use crate::install::ui::metrics::BAR_H;
use crate::install::ui::theme;

pub fn bar(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, done: u64, total: u64, colour: u32) {
    fb.fill_round(x, y, w, BAR_H, BAR_H / 2, theme::BAR_TRACK);
    if total == 0 {
        return;
    }
    let fill = ((w as u64).saturating_mul(done.min(total)) / total) as u32;
    if fill >= BAR_H {
        fb.fill_round(x, y, fill, BAR_H, BAR_H / 2, colour);
    } else if fill > 0 {
        fb.fill_rect(x, y + BAR_H / 4, fill, BAR_H / 2, colour);
    }
}
