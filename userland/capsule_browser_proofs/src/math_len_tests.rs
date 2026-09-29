// NONOS Operating System (AGPL-3.0-or-later)
//! Math functions (calc, min, max, clamp) in lengths, multi-value
//! shorthands split outside parentheses, signed margins, and viewport units
//! and media queries against the viewport the page is laid out at.

use crate::probe::Page;

const SITE: (u32, u32) = (1336, 760);

#[test]
fn a_clamp_padding_resolves_against_the_viewport() {
    let html = "<body style=\"margin:0\"><div id=d style=\"padding:0 clamp(24px, 5.2vw, 96px)\">\
                <span>word</span></div></body>";
    let p = Page::at(html, SITE);
    let x = p.word("word").x;
    assert!(x == 69 || x == 70, "5.2vw of 1336 is 69.5, text at {x}");
}

#[test]
fn a_clamp_font_size_takes_its_middle_value() {
    let html = "<h1 style=\"font-size:clamp(3rem, 7.6vw, 9.4rem)\">Big</h1>";
    let p = Page::at(html, SITE);
    let px = match &p.word("Big").content {
        crate::browser::layout::boxmodel::Content::Text { px, .. } => *px,
        _ => 0.0,
    };
    assert!((101.0..=102.0).contains(&px), "7.6vw of 1336 is 101.5, got {px}");
}

#[test]
fn a_function_inside_a_shorthand_stays_one_value() {
    let html = "<body style=\"margin:0\"><div id=d style=\"padding:calc(2px + 1vw) 5px\">\
                <span>in</span></div></body>";
    let p = Page::at(html, (800, 600));
    let w = p.word("in");
    assert_eq!((w.x, w.y), (5, 10), "calc(2px + 1vw) at 800 is 10");
}

#[test]
fn a_negative_margin_widens_the_box_past_its_container() {
    let html = "<body style=\"margin:0\"><div id=d style=\"margin:0 -20px\">x</div></body>";
    let r = Page::at(html, (800, 600)).rect("d");
    assert_eq!((r[0], r[2]), (-20, 840));
}

#[test]
fn a_negative_top_margin_pulls_the_box_up() {
    let html = "<body style=\"margin:0\"><div style=\"height:50px\"></div>\
                <div id=d style=\"margin-top:-30px;height:10px\"></div></body>";
    assert_eq!(Page::at(html, (800, 600)).rect("d")[1], 20);
}

#[test]
fn a_percentage_margin_takes_the_containing_width() {
    let html = "<body style=\"margin:0\"><div id=d style=\"margin-left:25%\">x</div></body>";
    assert_eq!(Page::at(html, (800, 600)).rect("d")[0], 200);
}

#[test]
fn viewport_units_and_media_queries_follow_the_viewport() {
    let html = "<style>@media (max-width: 900px) { #m { width: 123px } }</style>\
                <div id=v style=\"width:10vw;height:10vh\"></div><div id=m></div>";
    let p = Page::at(html, (800, 600));
    assert_eq!((p.rect("v")[2], p.rect("v")[3]), (80, 60));
    assert_eq!(p.rect("m")[2], 123);
    assert_ne!(Page::at(html, (1336, 760)).rect("m")[2], 123);
}
