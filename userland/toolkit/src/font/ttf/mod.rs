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

/* Antialiased vector text from a bundled TrueType face. Glyphs are
 * outlined by ab_glyph, rasterised once into a bounded coverage cache and
 * blended onto an ARGB8888 surface in integers. Metrics come straight from
 * the face, so layout and paint agree on advance widths. */

mod blend;
mod cache;
mod chrome;
mod clipped;
mod draw;
mod evict;
mod face;
mod fallback;
mod glyph;
mod gpos;
mod metrics;
mod raster;
mod rasterize;
mod readable;
mod sheared;
mod slant;
mod store;
mod target;
pub mod text_scale;
mod upright;
mod variants;

pub use cache::{clear_glyph_cache, glyph_cache_bytes};
pub use clipped::draw_text_clipped;
pub use face::builtin_face;
pub use raster::MAX_GLYPH_AREA;
pub use readable::MIN_UI_PX;
pub use slant::{draw_text_sheared, OBLIQUE};
pub use store::GLYPH_CACHE_BUDGET;

pub use ab_glyph::{FontRef, VariableFont};
pub use draw::{draw_text, draw_text_tracked};
pub use metrics::{
    ascent, ascent_with, line_height, line_height_with, measure, measure_spaced, measure_tracked,
    measure_with,
};
pub use variants::{draw_text_spaced, draw_text_with};
