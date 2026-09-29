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

use crate::browser::css::{BgSize, Shadow};

pub use super::content::Content;
use super::stack_key::Key;

/* One painted rectangle in absolute page coordinates. Border widths run
 * top, right, bottom, left; clip is [x0, y0, x1, y1] in page coordinates. */
pub struct Fragment {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub bg: u32,
    pub border: [u32; 4],
    pub border_color: u32,
    pub href: Option<String>,
    pub content: Content,
    /* Paint order key (geom::stack). */
    pub z: Key,
    pub clip: Option<[i32; 4]>,
    /* Corner radii of the box that made the clip, when the clip is its
     * padding box: paint rounds the clip's corners by them. */
    pub clip_r: [u16; 4],
    /* Painted without the scroll offset when true (position:fixed). */
    pub fixed: bool,
    /* Sticky anchor (flow y, top offset) shared by the sticky subtree. */
    pub sticky: Option<(i32, i32)>,
    /* Effective opacity for everything this fragment paints. */
    pub alpha: u8,
    /* background-image url to fetch and paint behind the box content; with
     * `mask` it is a mask-image, whose alpha paints the background color. */
    pub bg_image: Option<String>,
    pub mask: bool,
    pub bg_size: BgSize,
    pub bg_repeat: bool,
    /* drop shadow painted behind the box. */
    pub shadow: Option<Shadow>,
    /* Corner radii in px: top-left, top-right, bottom-right, bottom-left. */
    pub radius: [u16; 4],
    /* DOM node behind this rect (0 = anonymous), for event hit-testing. */
    pub node: usize,
}
