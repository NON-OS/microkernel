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

use core::ops::Index;

use alloc::boxed::Box;
use alloc::string::String;

use crate::browser::css::{Computed, GridSpec, PseudoText};
use crate::browser::dom::node::Node;
use crate::browser::dom::Dom;

/* The cascade's styles by node id (a text node reads its parent's), and
 * the pseudo-elements of an element by its id (empty for most). */
pub(super) type Styles<'a> = &'a dyn Index<usize, Output = Computed>;
pub(super) type Pseudos<'a> = &'a dyn Index<usize, Output = [PseudoText]>;
/* Named-grid data, boxed on the few nodes that carry any. */
pub(super) type Grids<'a> = &'a [Option<Box<GridSpec>>];

/* Shared state of one box-tree build walk: the source DOM, the resolved
 * styles and the box budget counter. */
pub(super) struct Walk<'a, 'b> {
    pub dom: &'a Dom,
    pub styles: Styles<'a>,
    pub bg_images: &'a [Option<String>],
    pub svg_paint: &'a [Option<Box<str>>],
    pub grids: Grids<'a>,
    pub pseudos: Pseudos<'a>,
    pub count: &'b mut usize,
}

/* One element child under consideration: the node, its DOM id, the
 * parent's tag and its 1-based li ordinal within that parent. */
pub(super) struct ElementIn<'a> {
    pub c: &'a Node,
    pub ch: usize,
    pub parent_tag: &'a str,
    pub ordinal: u32,
}
