// NONOS Operating System (AGPL-3.0-or-later)
//! Every child element of a flex container is an item of its own,
//! inline-flex is inline-level, and an item's width is taken once.

use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);

/* Two spans are two items: space-between puts the second at the far end. */
#[test]
fn two_spans_are_two_items() {
    let p = Page::at(
        "<body style=margin:0><div style='display:flex;justify-content:space-between;width:400px'>\
         <span id=a>one</span><span id=b>two</span></div>",
        VP,
    );
    let (a, b) = (p.rect("a"), p.rect("b"));
    assert_eq!(a[0], 0, "first item at the start");
    assert_eq!(b[0] + b[2], 400, "second item ends at the far edge: {b:?}");
}

/* An inline-flex box is an atom on its line: as wide as its content plus
 * its padding, with the text after it beside it, not below. */
#[test]
fn inline_flex_is_content_plus_padding_on_its_line() {
    let p = Page::at(
        "<body style=margin:0><div><a id=pill style='display:inline-flex;padding:0 26px'>\
         How it works</a> tail</div>",
        VP,
    );
    let pill = p.rect("pill");
    let (how, works, tail) = (p.word("How"), p.word("works"), p.word("tail"));
    let content = works.x + works.w - how.x;
    assert_eq!(pill[2], content + 52, "pill {pill:?} content {content}");
    assert_eq!(how.x, pill[0] + 26, "text starts inside the left padding");
    assert_eq!(tail.y, how.y, "the trailing text shares the line");
    assert!(tail.x >= pill[0] + pill[2], "tail at {} overlaps the pill {pill:?}", tail.x);
}

/* Uppercase text is measured as it is drawn, so a flex item sized to its
 * content keeps its words on one line. */
#[test]
fn an_uppercase_flex_item_stays_on_one_line() {
    let p = Page::at(
        "<body style=margin:0><div style='display:flex'><span style='text-transform:uppercase;\
         letter-spacing:.06em'>scroll to explore</span><span>+</span></div>",
        VP,
    );
    let (a, b, c) = (p.word("SCROLL"), p.word("TO"), p.word("EXPLORE"));
    assert!(a.y == b.y && b.y == c.y, "wrapped: {} {} {}", a.y, b.y, c.y);
}

/* A 25% item in a 1200px row is 300 wide: its percentage is taken of the
 * row once, not again of its own slot. */
#[test]
fn a_percentage_flex_item_resolves_once() {
    let p = Page::at(
        "<body style=margin:0><div style='display:flex;flex-wrap:wrap;width:1200px'>\
         <div id=h style='width:50%;float:left'>half</div>\
         <div id=c style='width:25%;padding:0 5px;box-sizing:border-box'>quarter one</div></div>",
        VP,
    );
    assert_eq!(p.rect("c")[2], 300, "25% of 1200");
    assert_eq!(p.rect("h")[2], 600, "50% of 1200");
    assert_eq!(p.rect("c")[1], p.rect("h")[1], "both fit on one line");
}
