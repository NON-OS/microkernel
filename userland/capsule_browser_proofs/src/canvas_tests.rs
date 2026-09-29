// NONOS Operating System (AGPL-3.0-or-later)
//! The canvas under the page takes the root element's background, or
//! body's when the root sets none (CSS Backgrounds 3 section 2.11.2), and
//! the box it came from does not paint it a second time.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

#[test]
fn the_canvas_is_the_root_elements_background() {
    let p = Page::at("<html id=h style=\"background:#0a0b0d\"><body id=b>x</body></html>", VP);
    assert_eq!(p.doc.canvas_bg, 0xff0a_0b0d);
    assert_eq!(p.frag("h").bg, 0, "the root must not paint the canvas again");
}

#[test]
fn bodys_background_moves_to_the_canvas_when_the_root_has_none() {
    let p = Page::at("<html id=h><body id=b style=\"background:#123456\">x</body></html>", VP);
    assert_eq!(p.doc.canvas_bg, 0xff12_3456);
    assert_eq!(p.frag("b").bg, 0, "body's background paints once, as the canvas");
}

#[test]
fn with_a_root_background_body_keeps_its_own() {
    let html = "<html style=\"background:#000000\"><body id=b style=\"background:#ff0000\">x</body></html>";
    let p = Page::at(html, VP);
    assert_eq!(p.doc.canvas_bg, 0xff00_0000);
    assert_eq!(p.frag("b").bg, 0xffff_0000);
}

#[test]
fn a_page_with_no_background_leaves_the_canvas_unset() {
    let p = Page::at("<p>plain</p>", VP);
    assert_eq!(p.doc.canvas_bg, 0, "the painter's white fallback shows");
}

#[test]
fn body_overflow_goes_to_the_viewport_and_does_not_clip() {
    let html = "<html><body style=\"margin:0;height:100px;overflow:hidden\">\
                <div id=t style=\"height:2000px\">tall</div></body></html>";
    let p = Page::at(html, VP);
    assert!(p.frag("t").clip.is_none(), "the viewport never clips");
    assert!(p.doc.content_h >= 2000, "the page scrolls to the content's end");
}
