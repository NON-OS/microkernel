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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::Computed;

use super::geom::box_aux::BoxAux;

pub enum BoxKind {
    Block,
    Inline,
    /* Inline-level on the outside, block on the inside: it sits in a line box
     * like a word but sizes to its own width and height and lays its children
     * in a block context. */
    InlineBlock,
    Flex,
    Grid,
    Text(String),
    Image { src: String, alt: String },
}

impl BoxKind {
    /* Block-level boxes stack in flow and count as flex/grid items. */
    pub(super) fn block_level(&self) -> bool {
        matches!(self, BoxKind::Block | BoxKind::Flex | BoxKind::Grid)
    }
}

/* The explicit placement a grid item asked for, as grid lines: 1-based, a
 * negative line counting back from the end of the explicit grid, 0 where
 * that side is auto, with the span that applies when a side is auto. Line
 * and area names are turned into numbers at build, while the name tables
 * are at hand; layout, which knows the track counts, does the rest. */
#[derive(Clone, Copy)]
pub struct GridPlace {
    pub col: [i16; 2],
    pub row: [i16; 2],
    pub col_span: u16,
    pub row_span: u16,
}

/* One box in the layout tree. Text and Image boxes are leaves; href carries
 * the enclosing anchor so hit-testing survives layout, and dom_id ties the
 * box back to its DOM node for event dispatch (0 = anonymous). */
pub struct BoxNode {
    pub kind: BoxKind,
    pub style: Computed,
    pub href: Option<String>,
    pub dom_id: usize,
    /* background-image url captured from the cascade, painted behind content. */
    pub bg_image: Option<String>,
    /* Explicit grid placement when this box is a grid item that asked for one. */
    pub grid_place: Option<GridPlace>,
    pub children: Vec<BoxNode>,
    /* Static position and image size hints, filled by build and layout. */
    pub(super) aux: BoxAux,
}
