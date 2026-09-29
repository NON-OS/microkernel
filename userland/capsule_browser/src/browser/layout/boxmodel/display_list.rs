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

use crate::browser::css::BgSize;

pub use super::geom::fragment::{Content, Fragment};

pub type DisplayList = Vec<Fragment>;

pub struct BoxDocument {
    pub frags: DisplayList,
    pub content_h: u32,
    /* The canvas under the whole page: the root element's background, or
     * body's when the root has none. 0 when neither sets one. */
    pub canvas_bg: u32,
    pub canvas_bg_image: Option<CanvasImage>,
    /* <img> sources laid out before their natural size was known; one
     * arriving is worth a relayout. */
    pub unsized_imgs: Vec<String>,
}

/* A background image the canvas tiles, taken from the root or body. */
pub struct CanvasImage {
    pub url: String,
    pub size: BgSize,
    pub repeat: bool,
}

impl BoxDocument {
    /* Fragments paint front to back, so the last hit wins. */
    pub fn link_at(&self, x: i32, y: i32) -> Option<&str> {
        for f in self.frags.iter().rev() {
            if let Some(href) = f.href.as_deref() {
                if x >= f.x && x < f.x + f.w && y >= f.y && y < f.y + f.h {
                    return Some(href);
                }
            }
        }
        None
    }

    /* Topmost DOM node under the point, for event dispatch. */
    pub fn hit_node(&self, x: i32, y: i32) -> Option<usize> {
        for f in self.frags.iter().rev() {
            if f.node != 0 && x >= f.x && x < f.x + f.w && y >= f.y && y < f.y + f.h {
                return Some(f.node);
            }
        }
        None
    }
}
