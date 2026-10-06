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

use alloc::boxed::Box;

use nonos_toolkit::font::ttf::{FontRef, VariableFont};

use super::face::Face;

impl Face {
    /// A copy of variable face `base` set to CSS weight `w` on its wght
    /// axis, over bytes of its own: the glyph cache knows a face by the
    /// address of its data, so each weight keeps its glyphs apart. None
    /// when the face has no weight axis.
    pub fn weighted(base: &Face, w: u16) -> Option<Face> {
        base.wght?;
        /* SAFETY: `base.data` stays allocated while `base` lives, and the
        caller holds `base` for the whole copy. */
        let bytes: Box<[u8]> = Box::from(unsafe { &*base.data });
        let mut face = Face::parse(base.key, bytes)?;
        face.weight = w;
        face.font.set_variation(b"wght", w as f32).then_some(face)
    }
}

/// The default of a face's weight axis, None when it has none.
pub(super) fn wght_default(font: &FontRef) -> Option<f32> {
    font.variations().iter().find(|a| a.tag == *b"wght").map(|a| a.default_value)
}
