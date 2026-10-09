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

//! SVG paint servers, <use> and clip paths.

use super::svg_paint_tests::{near, px, render};

#[test]
fn radial_gradients_use_and_clip_paths_resolve() {
    let d = render(
        "<defs><radialGradient id='r' gradientUnits='userSpaceOnUse' cx='50' cy='50' r='50'>\
         <stop offset='0' stop-color='white'/><stop offset='1' stop-color='black'/></radialGradient>\
         <clipPath id='c'><rect x='0' y='0' width='50' height='100'/></clipPath>\
         <rect id='box' width='100' height='100' fill='url(#r)'/></defs>\
         <use href='#box' clip-path='url(#c)'/><circle cx='75' cy='50' r='10' fill='red' fill-opacity='.5'/>",
    );
    assert!(near(px(&d, 49, 50), 0xffffffff, 12), "centre is white: {:08x}", px(&d, 49, 50));
    assert!(near(px(&d, 1, 50), 0xff050505, 12), "edge is dark: {:08x}", px(&d, 1, 50));
    assert_eq!(px(&d, 60, 20), 0, "the clip hides the right half");
    assert!(near(px(&d, 75, 50), 0x80ff0000, 3), "half-opaque red: {:08x}", px(&d, 75, 50));
}
