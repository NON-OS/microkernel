// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! An inline-block is laid out at the origin and then moved to its place on
//! the line; the clips inside it move with it. An axis it does not clip stays
//! open, and an edge taken from an ancestor's clip stays in page space.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

/* Whether a clip leaves any area at all. */
fn open(c: Option<[i32; 4]>) -> bool {
    c.is_none_or(|c| c[0] < c[2] && c[1] < c[3])
}

#[test]
fn a_one_axis_clip_on_an_inline_block_keeps_its_text_visible() {
    let html = "<body style=\"margin:0\"><p style=\"margin:40px 0 0 100px\">\
                <span style=\"display:inline-block;overflow-x:hidden;width:120px\">\
                visible</span></p></body>";
    let p = Page::at(html, VP);
    let f = p.word("visible");
    assert!(open(f.clip), "clip {:?}", f.clip);
    assert_eq!(f.clip.map(|c| (c[0], c[2])), Some((100, 220)));
}

#[test]
fn an_inline_block_inside_a_clipping_flex_item_is_not_moved_twice() {
    let html = "<body style=\"margin:0\"><div style=\"display:flex;width:600px\">\
                <b style=\"width:200px;flex:none\">x</b>\
                <span style=\"flex:1;overflow:hidden;white-space:nowrap\">\
                <span style=\"display:inline-block;overflow:hidden;max-width:100%\">\
                message</span></span></div></body>";
    let p = Page::at(html, VP);
    let f = p.word("message");
    assert!(open(f.clip), "clip {:?}", f.clip);
    assert!(f.clip.is_some_and(|c| c[0] <= f.x && f.x + f.w <= c[2]), "{:?} {}", f.clip, f.x);
}
