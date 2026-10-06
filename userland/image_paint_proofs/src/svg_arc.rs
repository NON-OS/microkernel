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

//! Inline SVG through the HTML tree builder, the serializer and the
//! rasterizer: the nonos.software hero arc, gradient references, and the
//! viewport CSS gives an SVG with no viewBox.
use std::string::String;

use crate::browser::image::{ingest, note_img_size, Store};
use crate::browser::layout::boxmodel::Content;
use crate::shim::{data_uri, render};

/* The hero markup as served, with `stops` and `stroke` substituted, in a
 * 400 x 400 box. */
pub(crate) fn arc(stops: [&str; 2], stroke: &str) -> String {
    std::format!(
        "<svg class=\"arc\" style=\"width:400px;height:400px\" viewBox=\"0 0 1000 1000\" preserveAspectRatio=\"none\" \
         aria-hidden=\"true\"><defs><linearGradient id=\"arc-ink\" x1=\"0\" y1=\"0\" \
         x2=\"0.35\" y2=\"1\"><stop offset=\"0\" stop-color=\"{}\"/><stop offset=\"1\" \
         stop-color=\"{}\"/></linearGradient></defs><path d=\"M-40 60 C 260 40, 640 240, \
         640 1040\" fill=\"none\" stroke=\"{stroke}\" stroke-width=\"46\" \
         stroke-linecap=\"round\"/></svg>",
        stops[0], stops[1]
    )
}

/* The inline SVG in `body` laid out, then rasterized at its box as the
 * device queue does: (box w, box h, serialized markup, ARGB pixels). */
pub(crate) fn raster(body: &str) -> (u32, u32, String, std::vec::Vec<u32>) {
    let doc = render(&std::format!("<!doctype html><html><body>{body}</body></html>"), 1336);
    let (w, h, src) = doc
        .frags
        .iter()
        .find_map(|f| match &f.content {
            Content::Image { src, .. } => Some((f.w as u32, f.h as u32, src.clone())),
            _ => None,
        })
        .expect("an inline svg box");
    let mut s = Store::new();
    note_img_size(&mut s, &src, w, h);
    ingest(&mut s, &src, &data_uri(&src).expect("data: payload"));
    let d = s.ready(&src).expect("rasterized");
    assert_eq!((d.w, d.h), (w, h), "rasterized at the box");
    (w, h, String::from(&src["data:image/svg+xml,".len()..]), d.px.clone())
}

pub(crate) fn px(p: &[u32], w: u32, x: u32, y: u32) -> u32 {
    p[(y * w + x) as usize]
}

/* On the path: the cubic at t = 0.5 is (412.5, 242.5) in the viewBox, so
 * (165, 97) at 400 x 400; stroke 46 units is 18.4 px wide there. */
pub(crate) const ON: (u32, u32) = (165, 97);
pub(crate) const OFF: (u32, u32) = (380, 20);
