// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The CSS-wide keywords: unset, initial on a property that does not
//! inherit, inherit on one that does, and revert, which goes back to the UA
//! sheet. Each passes over the declarations it overrides, and a keyword on
//! a longhand after its shorthand puts that one longhand back.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

#[test]
fn unset_on_flex_basis_after_the_flex_shorthand_sizes_the_item_by_content() {
    let css = ".row{display:flex;width:600px}.t{flex:0;flex-basis:unset}";
    let html =
        format!("<style>{css}</style><div class=row><div class=t id=t>NON-OS / kernel</div></div>");
    assert!(Page::at(&html, VP).rect("t")[2] > 100, "squeezed to its narrowest word");
}

#[test]
fn unset_drops_an_earlier_width() {
    let html = "<style>#b{width:100px}#b{width:unset}</style><div id=b>x</div>";
    assert_eq!(Page::at(html, VP).rect("b")[2], 800 - 16);
}

#[test]
fn a_keyword_on_a_shorthand_drops_its_longhands() {
    let html = "<style>#b{margin-left:50px}#b{margin:unset}</style><div id=b>x</div>";
    assert_eq!(Page::at(html, VP).rect("b")[0], 8);
}

#[test]
fn revert_keeps_the_ua_display() {
    let html =
        "<style>div{display:inline}div{display:revert}</style><div id=a>a</div><div id=b>b</div>";
    let p = Page::at(html, VP);
    assert!(p.rect("b")[1] > p.rect("a")[1], "a block goes on its own line");
}

#[test]
fn an_important_declaration_outlives_a_later_normal_keyword() {
    let html = "<style>#b{width:100px!important}#b{width:unset}</style><div id=b>x</div>";
    assert_eq!(Page::at(html, VP).rect("b")[2], 100);
}

#[test]
fn inherit_on_color_takes_the_parents() {
    let html = "<style>p{color:rgb(1,2,3)}b{color:red}b{color:inherit}</style><p><b id=b>x</b></p>";
    let p = Page::at(html, VP);
    let crate::browser::layout::boxmodel::Content::Text { color, .. } = p.word("x").content else {
        panic!("x is not text")
    };
    assert_eq!(color & 0xff_ffff, 0x01_0203);
}
