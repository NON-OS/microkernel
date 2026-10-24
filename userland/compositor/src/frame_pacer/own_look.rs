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

//! An output's look laid over its own copy of the true-colour canvas, all
//! but where the indicator band landed, which keeps its true colours
//! (`display::mirror_rule`).

use super::indicator_band::band;
use super::look::{apply, Look};
use crate::display::mirror_rule::around;
use crate::state::damage::Rect;
use crate::sw_blitter::{fitted, Surface};

/// `rect` of `dst`, a copy of a `canvas`-sized canvas, through `look`.
pub fn lay(dst: Surface, rect: Rect, canvas: (u32, u32), look: &Look) {
    let hole = fitted(canvas, (dst.width, dst.height), band(canvas.0, canvas.1));
    for r in around(rect, hole).into_iter().flatten() {
        apply(dst, r, look);
    }
}
