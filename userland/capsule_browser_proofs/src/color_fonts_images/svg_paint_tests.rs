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

//! SVG paint servers and references: gradients, use and defs, clip paths
//! and opacity, and the natural size a data: URI image reports.

use crate::browser::image::decode::{decode_body, natural};
use crate::browser::image::Decoded;

fn svg(body: &str) -> String {
    format!("<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100' width='100' height='100'>{body}</svg>")
}

pub(super) fn render(body: &str) -> Decoded {
    decode_body(svg(body).as_bytes(), (100, 100)).expect("renders")
}

pub(super) fn px(d: &Decoded, x: u32, y: u32) -> u32 {
    d.px[(y * d.w + x) as usize]
}

pub(super) fn near(got: u32, want: u32, tol: u32) -> bool {
    [24u32, 16, 8, 0].iter().all(|s| ((got >> s) & 0xff).abs_diff((want >> s) & 0xff) <= tol)
}

#[test]
fn a_ten_pixel_data_uri_is_ten_by_ten() {
    let uri = "<svg xmlns='http://www.w3.org/2000/svg' width='10' height='10'><rect width='10' height='10'/></svg>";
    assert_eq!(natural(uri.as_bytes()), Some((10, 10)));
    let d = decode_body(uri.as_bytes(), (0, 0)).expect("renders");
    assert_eq!((d.w, d.h), (10, 10));
}

#[test]
fn a_linear_gradient_runs_across_the_box() {
    let d = render(
        "<defs><linearGradient id='g'><stop offset='0' stop-color='#ff0000'/>\
         <stop offset='100%' stop-color='#0000ff'/></linearGradient></defs>\
         <rect x='0' y='0' width='100' height='100' fill='url(#g)'/>",
    );
    assert!(near(px(&d, 0, 50), 0xffff0000, 8), "{:08x}", px(&d, 0, 50));
    assert!(near(px(&d, 99, 50), 0xff0000ff, 8), "{:08x}", px(&d, 99, 50));
    assert!(near(px(&d, 50, 50), 0xff800080, 10), "{:08x}", px(&d, 50, 50));
}

#[test]
fn a_gradient_stroke_yields_pixels() {
    let d = render(
        "<linearGradient id='s' x1='0' y1='0' x2='0' y2='1'><stop stop-color='lime'/>\
         <stop offset='1' stop-color='yellow' stop-opacity='.5'/></linearGradient>\
         <path d='M10 10 L90 90' stroke='url(#s)' stroke-width='8' fill='none'/>",
    );
    assert!(
        px(&d, 50, 50) >> 24 > 0x80 && px(&d, 50, 50) & 0x00ff_0000 > 0,
        "{:08x}",
        px(&d, 50, 50)
    );
    assert_eq!(px(&d, 90, 10), 0, "nothing off the line");
}
