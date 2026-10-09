// NONOS Operating System (AGPL-3.0-or-later)
//! Generated content: an empty content string still makes a box, the alt
//! text after '/' is not drawn, attr() reads the element, var() resolves
//! inside content, and ::marker styles the list marker.

use crate::probe::Page;
use crate::render::texts;

const VP: (u32, u32) = (800, 600);

#[test]
fn empty_content_makes_a_seven_pixel_box() {
    let html = "<style>.n::before{content:'';display:inline-block;width:7px;height:7px;background:#ff0000}</style>\
                <p class=n>label</p>";
    let p = Page::at(html, VP);
    let hit = p.doc.frags.iter().find(|f| f.w == 7 && f.h == 7 && f.bg == 0xffff_0000);
    assert!(hit.is_some(), "the empty ::before paints its 7x7 box");
    let html = "<style>.f{display:inline-flex;align-items:center;gap:10px}.f::before{content:\"\";width:7px;height:7px;background:#ff0000}</style>\
                <p class=f>label</p>";
    let p = Page::at(html, VP);
    let hit = p.doc.frags.iter().find(|f| f.w == 7 && f.h == 7 && f.bg == 0xffff_0000);
    assert!(hit.is_some(), "a flex host blockifies its inline ::before");
    let html = "<style>.n::before{content:none;display:block;height:7px;background:#ff0000}</style><p class=n>x</p>";
    let p = Page::at(html, VP);
    assert!(!p.doc.frags.iter().any(|f| f.bg == 0xffff_0000), "content:none makes nothing");
}

#[test]
fn content_alt_text_is_not_drawn() {
    let p = Page::at("<style>.e::before{content:'[' / ''}.e::after{content:']' / 'end'}</style><p class=e>edit</p>", VP);
    let words: alloc::vec::Vec<&str> = texts(&p.doc).into_iter().map(|t| t.3).collect();
    assert!(words.contains(&"["), "{words:?}");
    assert!(words.contains(&"]"), "{words:?}");
    assert!(!words.iter().any(|w| w.contains("end") || w.contains('/')), "{words:?}");
}

#[test]
fn attr_and_var_feed_content() {
    let html = "<style>.a::after{content:attr(data-x)}.v{--t:'via-var'}.v::before{content:var(--t)}</style>\
                <p class=a data-x=hello>w</p><p class=v>z</p>";
    let p = Page::at(html, VP);
    p.word("hello");
    p.word("via-var");
}

#[test]
fn counters_and_quotes_generate_text() {
    let html = "<style>ol.c{counter-reset:k}ol.c li{counter-increment:k;list-style:none}\
                ol.c li::before{content:counter(k) \". \"}q::before{content:open-quote}</style>\
                <ol class=c><li>a</li><li>b</li></ol><p><q>x</q></p>";
    let p = Page::at(html, VP);
    p.word("2.");
}

#[test]
fn marker_takes_its_pseudo_style() {
    let html = "<style>li::marker{color:#ff0000}</style><ul><li>item</li></ul>";
    let p = Page::at(html, VP);
    let red = p.doc.frags.iter().any(|f| match &f.content {
        crate::browser::layout::boxmodel::Content::Text { color, text, .. } => {
            *color == 0xffff_0000 && text != "item"
        }
        _ => false,
    });
    assert!(red, "the bullet paints in the ::marker colour");
}
