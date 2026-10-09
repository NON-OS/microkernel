// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Layout on boxes narrower than the smallest item. A flex line 1px wide
//! made clamp(16, 1) panic in flex_row, which took the browser down on
//! wiki.archlinux.org at every window width.

use crate::render::render;

#[test]
fn a_wrapping_flex_line_narrower_than_an_item_lays_out() {
    let doc = render(
        "<div style=\"display:flex;flex-wrap:wrap;width:1px\"><div>abc</div><div style=\"width:40px\">d</div></div>",
        800,
    );
    assert!(!doc.frags.is_empty());
}

#[test]
fn every_viewport_from_one_to_forty_pixels_lays_out() {
    let page = "<div style=\"display:flex;flex-wrap:wrap\"><div>one</div><div style=\"flex-basis:50%\">two</div></div>\
<div style=\"display:grid;grid-template-columns:repeat(auto-fill,minmax(100px,1fr))\"><div>a</div></div>\
<div style=\"width:10%;padding:30px\">b</div>";
    for w in 1..=40 {
        assert!(!render(page, w).frags.is_empty(), "width {w}");
    }
}

#[test]
fn a_grid_item_placed_past_the_last_line_stays_in_the_grid() {
    let doc = render(
        "<style>.g{display:grid;grid-template-columns:[a] 10px [b] 10px [c]}.i{grid-column:c/9}</style><div class=g><div class=i>x</div></div>",
        400,
    );
    assert!(!doc.frags.is_empty());
}
