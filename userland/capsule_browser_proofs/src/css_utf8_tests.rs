// NONOS Operating System (AGPL-3.0-or-later)
//! A CSS value whose byte at a function-name boundary sits inside a
//! multi-byte character. Every one of these used to slice the str at that
//! byte and panic, which takes the whole browser down on one page.

use crate::render::render;

fn renders(style: &str) {
    let html = alloc::format!("<div style=\"{style}\">a</div>");
    let doc = render(&html, 800);
    assert!(!doc.frags.is_empty(), "{style}");
}

#[test]
fn a_size_with_a_wide_char_at_byte_five_renders() {
    renders("width:1234\u{e9}x");
}

#[test]
fn a_grid_value_with_a_wide_char_at_byte_seven_renders() {
    renders("display:grid;grid-template-columns:abcdef\u{e9}");
    renders("display:grid;grid-template-columns:repeat\u{e9}(auto-fill,10px)");
}

#[test]
fn a_track_with_a_wide_char_at_byte_seven_renders() {
    renders("display:grid;grid-template-columns:repeat(auto-fill,minmax\u{e9}(1px,2px))");
}
