// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! A box that clips its overflow with rounded corners hands those radii,
//! less its borders, to the content it clips, so paint rounds them too;
//! a clip that is not exactly that box's is square.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

#[test]
fn an_image_in_a_rounded_card_takes_the_cards_corners() {
    let html = "<div style=\"width:300px;height:200px;overflow:hidden;border-radius:24px\">\
                <img id=i src=a.jpg width=300 height=200></div>";
    assert_eq!(Page::at(html, VP).frag("i").clip_r, [24; 4]);
}

#[test]
fn a_border_narrows_the_inner_corners() {
    let html = "<div style=\"width:300px;height:200px;overflow:hidden;border-radius:24px;\
                border:4px solid red\"><img id=i src=a.jpg width=300 height=200></div>";
    assert_eq!(Page::at(html, VP).frag("i").clip_r, [20; 4]);
}

#[test]
fn a_clip_inside_the_card_is_square() {
    let html = "<div style=\"width:300px;height:200px;overflow:hidden;border-radius:24px\">\
                <div style=\"width:100px;height:50px;overflow:hidden\">\
                <img id=i src=a.jpg width=300 height=200></div></div>";
    assert_eq!(Page::at(html, VP).frag("i").clip_r, [0; 4]);
}
