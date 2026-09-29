// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Block-in-inline: a block inside an inline element keeps its own box, the
//! inline breaks around it, and a flex item that holds a block stays one item.

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
