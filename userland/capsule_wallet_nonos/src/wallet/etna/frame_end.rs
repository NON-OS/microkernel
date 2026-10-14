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

//! The frame's second pass: the bar, the footer and the status line.

use nonos_app_skeleton::PaintBuffer;

use super::super::frame_bar::bar;
use super::super::frame_foot::foot;
use super::super::frame_spec::{FrameLayout, FrameSpec};
use super::super::tokens::{BAR_H, INK};
use super::column_x;

/// Bar, footer and status line, over the content.
pub fn end(fb: &mut PaintBuffer, spec: &FrameSpec, out: &mut FrameLayout) {
    let x0 = column_x(fb);
    fb.fill_rect(x0, 0, super::super::current::now().column, BAR_H + 1, INK);
    out.back = bar(fb, x0, spec);
    foot(fb, x0, spec, out);
}
