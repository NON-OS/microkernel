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

//! The fill behind a cell: the selection, a search match, its own
//! background, or none where the theme's backdrop shows through.

use nonos_toolkit::paint::mixer::mix;
use nonos_vt::cell::attr;
use nonos_vt::{Cell, Color};

use super::area::{Frame, Shade, OPAQUE};
use crate::term::select::in_span;

/// The fill behind a cell, or none where the theme's backdrop shows.
pub(super) fn background(
    f: &Frame,
    cell: &Cell,
    bg: u32,
    abs: u64,
    col: usize,
    shade: Shade,
) -> Option<u32> {
    if shade.selection.is_some_and(|s| s.contains(abs, col)) {
        return Some(OPAQUE | mix(f.t.bg, f.t.accent, 90));
    }
    if in_span(shade.found, abs, col) {
        return Some(OPAQUE | mix(f.t.bg, f.t.run, 150));
    }
    let plain = cell.bg == Color::Default && cell.attr & attr::INVERSE == 0;
    if plain && !f.vt.modes.reverse_video {
        return None;
    }
    Some(OPAQUE | bg)
}
