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

//! The type scale. Each screen class has six faces and the mark; the class follows the
//! live mode, so text keeps its size relative to the screen from 800x600 to
//! 4K, and every spacing is a multiple of `unit`.

use super::atlas::{face, Face};
use crate::display::gop::get_dimensions;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Style {
    /// JetBrains Mono: labels in capitals, hashes, versions, numbers.
    Mono = 0,
    Caption = 1,
    Body = 2,
    Label = 3,
    Heading = 4,
    Display = 5,
}

/// Six text faces and the mark, per screen class.
const PER_CLASS: usize = 7;

/// 0 below 900 lines, 1 below 1000, 2 below 1700, 3 above, and never more
/// than the width allows, so a line of body text still fits. A 1080p screen
/// takes class 2: 23 px body text, which reads at arm's length on a laptop.
pub fn class() -> usize {
    let (w, h) = get_dimensions();
    let by_h = [900, 1000, 1700].iter().filter(|&&t| h >= t).count();
    let by_w = [1200, 1700, 2500].iter().filter(|&&t| w >= t).count();
    by_h.min(by_w)
}

pub fn face_of(s: Style) -> Option<Face> {
    face(class() * PER_CLASS + s as usize)
}

/// The mark's face: the brand Ø (0xD8) and its glow (0x04).
pub fn mark_face() -> Option<Face> {
    face(class() * PER_CLASS + PER_CLASS - 1)
}

/// The spacing unit: a third of the body size.
pub fn unit() -> u32 {
    face_of(Style::Body).map_or(5, |f| (f.px / 3).max(4))
}
