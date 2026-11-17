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

//! The document tab strip above the code pane, drawn as the shared `Tabs`
//! with the view controls at its right. Painting and the click both build the
//! strip from the documents, so a click lands on the control drawn under it.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Pointer, Rect};

use super::layout::{tabbar_h, titlebar_h};
use super::state::State;
use super::strip::Strip;
use super::theme;

pub(super) fn basename(doc: &State) -> &str {
    let path = core::str::from_utf8(&doc.path[..doc.path_len]).unwrap_or("");
    let name = path.rsplit('/').next().unwrap_or(path);
    if name.is_empty() {
        "untitled"
    } else {
        name
    }
}

/// The band the strip fills: from the code pane's left edge to the window's
/// right edge, under the menu bar.
pub(super) fn band(pane_x: u32, width: u32) -> Rect {
    Rect { x: pane_x, y: titlebar_h(), w: width.saturating_sub(pane_x), h: tabbar_h() }
}

/// Lends `f` the strip these documents make in `band`.
pub(super) fn with_strip<R>(
    docs: &[State],
    active: usize,
    band: Rect,
    f: impl FnOnce(&Strip) -> R,
) -> R {
    let names: Vec<&str> = docs.iter().map(basename).collect();
    let dirty: Vec<bool> = docs.iter().map(|d| d.dirty).collect();
    f(&Strip { names: &names, dirty: &dirty, active, band })
}

pub(super) fn paint_tabs(
    fb: &mut PaintBuffer,
    docs: &[State],
    active: usize,
    band: Rect,
    ptr: &Pointer,
) {
    fb.fill_rect(band.x, band.y, band.w, band.h, theme::active().tabbar_bg);
    with_strip(docs, active, band, |s| s.draw(fb, ptr));
}
