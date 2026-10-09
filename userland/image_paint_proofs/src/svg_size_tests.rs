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

//! Inline SVG boxes: width/height attributes (zero included) stand in for
//! auto CSS sizes, one definite side and a viewBox give the other, and an
//! SVG with neither takes the 300x150 default replaced size.
use std::string::String;
use std::vec::Vec;

use crate::browser::layout::boxmodel::Content;
use crate::shim::render;

/* (width, height, src) of every image fragment, in document order. */
fn images(body: &str) -> Vec<(i32, i32, String)> {
    let doc = render(&std::format!("<!doctype html><html><body>{body}</body></html>"), 1336);
    let pick = |f: &crate::browser::layout::boxmodel::Fragment| match &f.content {
        Content::Image { src, .. } => Some((f.w, f.h, src.clone())),
        _ => None,
    };
    doc.frags.iter().filter_map(pick).collect()
}

fn sizes(body: &str) -> Vec<(i32, i32)> {
    images(body).into_iter().map(|(w, h, _)| (w, h)).collect()
}

#[test]
fn attributes_and_the_viewbox_size_an_inline_svg() {
    let r = "<rect width='10' height='10'/>";
    let cases = [
        (std::format!("<svg width='0' height='0' style='position:absolute'>{r}</svg>"), (0, 0)),
        (std::format!("<svg viewBox='0 0 100 20' style='width:200px'>{r}</svg>"), (200, 40)),
        (std::format!("<svg>{r}</svg>"), (300, 150)),
        (std::format!("<svg width='120'>{r}</svg>"), (120, 150)),
        (std::format!("<svg height='30' viewBox='0 0 100 20'>{r}</svg>"), (150, 30)),
        (std::format!("<svg viewBox='0,0,50,100' style='height:80px'>{r}</svg>"), (40, 80)),
        (std::format!("<svg width='24px' height='24.4px'>{r}</svg>"), (24, 24)),
        (std::format!("<svg width='0' viewBox='0 0 100 20'>{r}</svg>"), (0, 0)),
    ];
    for (html, want) in cases {
        assert_eq!(sizes(&html), [want], "{html}");
    }
}

#[test]
fn a_zero_sized_img_takes_no_room() {
    assert_eq!(sizes("<img src='a.png' width='0' height='0'>"), [(0, 0)]);
    assert_eq!(sizes("<img src='a.png' width='16px' height='9'>"), [(16, 9)]);
}

#[test]
fn deeply_nested_svg_content_is_serialized_whole() {
    let rects: String =
        (0..60).map(|i| std::format!("<rect x='{i}' width='1' height='1'/>")).collect();
    let found = images(&std::format!("<svg width='60' height='10'>{rects}</svg>"));
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].2.matches("<rect").count(), 60, "every nested rect survives");
}
