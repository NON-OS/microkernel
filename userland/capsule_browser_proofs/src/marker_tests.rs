// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! List markers belong to list items: an li laid out as a block gets its
//! bullet or ordinal; one displayed inline (a horizontal list with its own
//! separators) or as a flex container gets none.

use crate::render::{render, texts};

fn bullets(html: &str) -> usize {
    let doc = render(html, 800);
    texts(&doc).iter().filter(|t| t.3.starts_with('\u{2022}')).count()
}

#[test]
fn a_plain_list_item_has_its_bullet() {
    assert_eq!(bullets("<ul><li>one</li><li>two</li></ul>"), 2);
}

#[test]
fn an_inline_list_item_has_no_bullet() {
    let html = "<style>li{display:inline}li:after{content:\" \\00b7 \"}</style>\
                <ul><li>Kernel</li><li>Drivers</li></ul>";
    assert_eq!(bullets(html), 0);
}

#[test]
fn a_flex_list_item_has_no_bullet() {
    assert_eq!(bullets("<ul><li style=\"display:flex\">tab</li></ul>"), 0);
}
