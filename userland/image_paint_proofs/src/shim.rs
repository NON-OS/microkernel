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

//! Public doors to engine items the capsule keeps crate-visible, so the
//! proofs (and nothing else) can call them by name.
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use crate::browser::css::{collect_css, compute};
use crate::browser::dom;
use crate::browser::layout::boxmodel::{build, layout, BoxDocument};

pub fn data_uri(uri: &str) -> Option<Vec<u8>> {
    crate::browser::image::data_uri_bytes(uri)
}

pub fn is_gradient(src: &str) -> bool {
    crate::browser::paint::grad::is_gradient(src)
}

pub fn paint_gradient(fb: &mut PaintBuffer, src: &str, rect: [i32; 4], clip: [i32; 4]) -> bool {
    crate::browser::paint::grad::paint_gradient(fb, src, rect, clip)
}

pub fn put_pixel(fb: &mut PaintBuffer, x: i32, y: i32, argb: u32) {
    crate::browser::paint::grad::put_pixel(fb, x, y, argb)
}

/* A page through the capsule's cascade, box build and layout. */
pub fn render(html: &str, viewport_w: u32) -> BoxDocument {
    let d = dom::parse(html.as_bytes());
    let css = collect_css(&d);
    let s = compute(&d, &css);
    let root = build(&d, &s.styles, &s.bg_images, &s.grids, &s.pseudos, &|_| None);
    layout(&root, (viewport_w, 760))
}
