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

//! The toolkit's `font` module as far as the bitmap text the service paints
//! with: the atlas, the glyph tables and the renderers. The TrueType half and
//! its fallback face need ab_glyph and stay out.

#[path = "../../toolkit/src/font/atlas.rs"]
pub mod atlas;
#[path = "../../toolkit/src/font/glyph.rs"]
pub mod glyph;
#[path = "../../toolkit/src/font/lower.rs"]
mod lower;
#[path = "../../toolkit/src/font/punct.rs"]
mod punct;
#[path = "../../toolkit/src/font/render/mod.rs"]
pub mod render;
#[path = "../../toolkit/src/font/upper.rs"]
mod upper;
