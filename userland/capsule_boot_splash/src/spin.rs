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

//! One turn of the status line's spinner.

use crate::layout::Area;
use crate::{scene, status};

/// The status line's next frame, and damage for that line alone: a whole
/// screen's damage every spin had the compositor recompose it for one line.
pub(crate) fn spin(comp: u32, base: u64, (w, h, spx): (u32, u32, usize), band: Area, frame: u32) {
    // SAFETY: `base` maps the splash's surface, `spx * h` words.
    let buf = unsafe { core::slice::from_raw_parts_mut(base as *mut u32, spx * h as usize) };
    status::status(buf, spx, w, h, band, frame);
    let _ = scene::damage(comp, 21, band);
}
