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

use super::chip::{chip, chip_w, CHIP_GAP, CHIP_H};
use super::tags::TagMap;
use super::theme::INK3;

const LABEL_PX: f32 = 14.0;

/// The tags on `path` as a wrapped chip band, or an honest note when there are
/// none. Chips wrap when the next measured pill would overrun `w`.
pub fn tag_band(fb: &mut PaintBuffer, tags: &TagMap, path: &str, x: u32, top: u32, w: u32) {
    let names = tags.tags_for(path);
    if names.is_empty() {
        let _ = fb.text_ttf(x as i32, top as i32, "No tags", INK3, LABEL_PX);
        return;
    }
    let mut cx = x;
    let mut y = top;
    for name in names {
        if cx > x && cx + chip_w(name) > x + w {
            cx = x;
            y += CHIP_H + CHIP_GAP;
        }
        cx += chip(fb, cx, y, name, false);
    }
}
