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

use alloc::vec::Vec;

use super::defs::Defs;
use super::raster::Raster;

/* Subtrees that define resources or content not rendered in place:
 * skipped whole so their geometry never paints. Gradients, clip paths and
 * symbols are drawn by reference; masks, patterns, filters, text and
 * embedded images are not drawn. */
const SKIPPED: &str = "defs symbol clipPath mask style linearGradient radialGradient pattern \
                       filter text metadata title desc image";

pub(super) fn skipped(name: &str) -> bool {
    SKIPPED.split_ascii_whitespace().any(|s| s == name)
}

pub(super) fn container(name: &str) -> bool {
    matches!(name, "g" | "a" | "svg" | "switch" | "symbol")
}

/// Painting one band: the definitions, the band, the clip masks of the
/// open groups, how deep `use` references nest, and how many more tags
/// may be visited, which bounds the work references can multiply.
pub(super) struct Walk<'d, 'r> {
    pub defs: &'d Defs<'d>,
    pub r: &'r mut Raster,
    pub masks: Vec<Raster>,
    pub depth: u32,
    pub budget: u32,
}
