// NONOS Operating System (AGPL-3.0-or-later)
//! Absolutely positioned and fixed boxes leave normal flow and are placed
//! against their containing block: the padding box of the nearest
//! positioned ancestor, or the viewport for a fixed box.

use crate::browser::layout::boxmodel::Content;
use crate::probe::Page;

#[test]
fn a_fixed_box_with_inset_zero_fills_the_viewport() {
    let p = Page::at("<p>text</p><div id=f style=\"position:fixed;inset:0\"></div>", (800, 600));
    assert_eq!(p.rect("f"), [0, 0, 800, 600]);
    assert!(p.frag("f").fixed);
}

#[test]
fn a_bottom_inset_anchors_the_box_to_the_bottom() {
    let html = "<body style=\"margin:0\"><div style=\"position:relative;height:800px\">\
                <div id=a style=\"position:absolute;bottom:10px;height:20px;left:0;right:0\"></div>\
                </div></body>";
    assert_eq!(Page::at(html, (800, 800)).rect("a"), [0, 770, 800, 20]);
}

#[test]
fn an_absolute_box_takes_no_space_in_flow() {
    let html = "<body style=\"margin:0\"><div id=a style=\"position:absolute\">ghost</div>\
                <div id=b>after</div></body>";
    let p = Page::at(html, (800, 600));
    assert_eq!(p.rect("b")[1], 0, "the absolute box before it took no room");
    assert_eq!(p.rect("a")[1], 0, "with every inset auto it sits at its static place");
}

#[test]
fn a_right_inset_places_a_shrink_to_fit_box() {
    let html = "<body style=\"margin:0\"><div style=\"position:relative;width:500px\">\
                <span id=a style=\"position:absolute;right:0;top:0\">tag</span></div></body>";
    let r = Page::at(html, (800, 600)).rect("a");
    assert_eq!(r[0] + r[2], 500, "right edge on the container's");
    assert!(r[2] > 0 && r[2] < 100, "shrinks to its word, {r:?}");
}

#[test]
fn an_absolute_image_still_paints() {
    let html = "<div style=\"position:relative;width:300px;height:200px\">\
                <img id=i src=\"a.png\" style=\"position:absolute;inset:0;width:100%;height:100%\"></div>";
    let p = Page::at(html, (800, 600));
    assert!(matches!(p.frag("i").content, Content::Image { .. }));
    assert_eq!((p.rect("i")[2], p.rect("i")[3]), (300, 200));
}

#[test]
fn top_percentages_take_the_containing_blocks_height() {
    let html =
        "<body style=\"margin:0\"><div style=\"position:relative;width:100px;height:400px\">\
                <div id=a style=\"position:absolute;top:50%;left:0;width:10px;height:10px\"></div>\
                </div></body>";
    assert_eq!(Page::at(html, (800, 600)).rect("a")[1], 200);
}
