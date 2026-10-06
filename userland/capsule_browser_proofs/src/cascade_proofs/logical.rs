// NONOS Operating System (AGPL-3.0-or-later)
//! Logical margin and padding properties map to the physical sides of a
//! horizontal, left-to-right page, so a sheet that resets the UA sheet's
//! em margins through margin-block-start does reset them; and only a
//! display:table-caption child of a table is laid out as its caption.

use super::probe::Styles;
use crate::probe::Page;

#[test]
fn logical_margins_and_padding_set_the_physical_sides() {
    let s = Styles::of(
        "<style>h3{margin-block-start:0;margin-block-end:4px}\
         #p{margin-inline:3px 5px;padding-block:2px;padding-inline-start:6px}\
         #q{margin-block:7px;margin-inline-end:calc(1px + 2px)}</style>\
         <h3 id=h>h</h3><p id=p>p</p><p id=q>q</p>",
    );
    let h = s.get("h");
    assert_eq!((h.margin_top, h.margin_bottom), (0, 4), "the UA em margins are reset");
    let p = s.get("p");
    assert_eq!((p.margin_left, p.margin_right), (3, 5));
    assert_eq!((p.pad_top, p.pad_bottom, p.pad_left), (2, 2, 6));
    let q = s.get("q");
    assert_eq!((q.margin_top, q.margin_bottom, q.margin_right), (7, 7, 3));
}

#[test]
fn only_a_table_caption_box_is_a_caption() {
    let html = "<body style=\"margin:0\"><table><caption id=c>cap</caption><tbody><tr>\
                <td id=d>cell</td></tr></tbody></table>";
    let p = Page::at(html, (800, 600));
    let (c, d) = (p.rect("c"), p.rect("d"));
    assert!(c[1] + c[3] <= d[1], "the caption sits above the rows: {c:?} {d:?}");
    /* A stray block inside a table is foster-parented before it (HTML
     * 13.2.6.1), as Chromium does: it shows above the table, no caption. */
    let html = "<body><table><tbody><tr><td id=d>cell</td></tr></tbody>\
                <div id=s style=\"display:block\">stray</div></table>";
    let p = Page::at(html, (800, 600));
    let parent = &p.dom.nodes[p.dom.nodes[p.node("s")].parent].tag;
    assert_eq!(parent, "body", "the parser moves the stray block out of the table");
    assert!(p.word("stray").y + p.word("stray").h <= p.rect("d")[1], "above the rows");
}
