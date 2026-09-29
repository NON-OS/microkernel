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

//! The background shorthand's colour and the box-shadow layers as the
//! cascade leaves them on the painted fragment.

use crate::probe::Page;

pub(super) fn page(css: &str) -> Page {
    Page::at(&format!("<style>{css}</style><div id=a class='a b'>x</div>"), (800, 600))
}

#[test]
fn background_colour_comes_from_the_final_layer_only() {
    let bg = |v: &str| page(&format!("#a{{background:{v}}}")).frag("a").bg;
    assert_eq!(bg("linear-gradient(#fff, #000), url(x.png) #123456"), 0xFF12_3456);
    assert_eq!(bg("color-mix(in srgb,#66ffff 22%,transparent)"), 0x3866_FFFF);
    assert_eq!(bg("url(/a/b.png) center/cover no-repeat rgba(10,11,15,.34)"), 0x570A_0B0F);
    assert_eq!(
        bg("linear-gradient(#fff, #000)"),
        0,
        "a colour inside a function is no layer colour"
    );
    assert_eq!(bg("#fff url(x.png), #000"), 0xFF00_0000, "only the last layer may carry one");
}

#[test]
fn the_shorthand_resets_an_earlier_colour() {
    let p = page(".a{background:red}.a.b{background:url(x.png) no-repeat}");
    assert_eq!(p.frag("a").bg, 0, "background: without a colour is transparent");
    let p = page(".a{background-color:red}.a.b{background-color:transparent}");
    assert_eq!(p.frag("a").bg, 0);
}

#[test]
fn an_inset_shadow_replaces_the_outer_one() {
    let p = page(".a{box-shadow:0 1px 0 #000}.a.b{box-shadow:inset 0 0 0 1px #fff}");
    let s = p.frag("a").shadow.expect("the inset shadow");
    assert_eq!(s.n, 1, "no outer layer is left behind");
    assert!(s.layers[0].inset);
    assert_eq!((s.layers[0].spread, s.layers[0].color), (1, 0xFFFF_FFFF));
}

#[test]
fn every_layer_is_kept_with_blur_and_spread() {
    let css = ".a{color:#0000ff;box-shadow:0 0 #0000, 0 0 0 1px rgba(0,0,0,.1), \
               inset 0 -2px 4px -1px, 0 10px 30px -5px #000}";
    let s = page(css).frag("a").shadow.expect("shadow layers");
    assert_eq!(s.n, 3, "the fully transparent layer paints nothing and is dropped");
    let l = |i: usize| s.layers[i];
    assert_eq!((l(0).dx, l(0).dy, l(0).blur, l(0).spread, l(0).color), (0, 0, 0, 1, 0x1A00_0000));
    assert!(l(1).inset && l(1).blur == 4 && l(1).spread == -1 && l(1).dy == -2);
    assert_eq!(l(1).color, 0xFF00_00FF, "no colour means currentColor");
    assert_eq!((l(2).dy, l(2).blur, l(2).spread, l(2).inset), (10, 30, -5, false));
}
