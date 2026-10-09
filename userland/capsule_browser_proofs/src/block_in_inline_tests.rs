// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Block-in-inline: a block inside an inline element keeps its own box, the
//! inline breaks around it (floats too, with no line box for the white space
//! between them), and a flex item that holds a block stays one item.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

#[test]
fn a_block_inside_an_unknown_element_keeps_its_width_and_flex_row() {
    let html = "<body style=\"margin:0\"><react-partial><div id=bar style=\"display:flex\">\
                <a id=a>Platform</a><a id=b>Solutions</a></div></react-partial></body>";
    let p = Page::at(html, VP);
    assert_eq!(p.rect("bar")[2], 800, "the block spans its container");
    assert_eq!(p.rect("a")[1], p.rect("b")[1], "flex items share one row");
    assert!(p.rect("b")[0] > p.rect("a")[0]);
}

#[test]
fn the_inline_breaks_around_the_block() {
    let html = "<body style=\"margin:0\"><span>before<div id=d>middle</div>after</span></body>";
    let p = Page::at(html, VP);
    let (b, m, a) = (p.word("before").y, p.word("middle").y, p.word("after").y);
    assert!(b < m && m < a, "three lines in order: {b} {m} {a}");
    assert_eq!(p.rect("d")[2], 800);
}

#[test]
fn a_flex_item_holding_a_block_stays_one_item() {
    let html = "<body style=\"margin:0\"><div style=\"display:flex\">\
                <a id=tab><span style=\"display:block\">Code</span></a><a id=next>Issues</a></div></body>";
    let p = Page::at(html, VP);
    assert_eq!(p.rect("tab")[1], p.rect("next")[1]);
    assert!(p.rect("next")[0] < 200, "the tab stayed narrow: {:?}", p.rect("next"));
}

#[test]
fn floats_in_an_inline_list_sit_in_a_row_and_size_their_container() {
    let html = "<body style=\"margin:0\"><div style=\"display:flex\"><div style=\"flex:auto\">t</div>\
                <div id=box style=\"flex-shrink:0\"><ul style=\"display:inline;margin:0;padding:0\">\n\
                <li id=a style=\"float:left\">Notifications</li>\n<li id=b style=\"float:left\">Fork</li>\n\
                </ul></div></div></body>";
    let p = Page::at(html, VP);
    let (a, b, bx) = (p.rect("a"), p.rect("b"), p.rect("box"));
    assert_eq!(a[1], b[1], "one row");
    assert!(bx[2] >= a[2] + b[2], "the container holds both: {bx:?} {a:?} {b:?}");
}
