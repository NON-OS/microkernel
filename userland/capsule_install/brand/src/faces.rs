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

//! The brand's faces, per nonos.software/brand: Geist for text, at 400 for
//! body and 500 for headlines, and JetBrains Mono for labels, hashes and
//! numbers. Built by assets/make_assets.py.

use nonos_toolkit::font::ttf::FontRef;

static REGULAR: &[u8] = include_bytes!("../assets/Geist-Regular.ttf");
static MEDIUM: &[u8] = include_bytes!("../assets/Geist-Medium.ttf");
static MONO: &[u8] = include_bytes!("../assets/JetBrainsMono-Regular.ttf");

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Body,
    Headline,
    Mono,
}

/// The face, parsed from its bytes; None if the bytes do not parse, and
/// every draw with it is then a no-op.
pub fn font(face: Face) -> Option<FontRef<'static>> {
    let bytes = match face {
        Face::Body => REGULAR,
        Face::Headline => MEDIUM,
        Face::Mono => MONO,
    };
    FontRef::try_from_slice(bytes).ok()
}
