// NONOS Operating System (AGPL-3.0-or-later)
//! Inline layout keeps the white space the source has, and only that:
//! element boundaries add no space, collapsible spaces collapse to one,
//! preserved newlines break lines, and an underline runs unbroken.

use crate::probe::Page;
use crate::render::texts;

const VP: (u32, u32) = (1336, 800);

fn at(html: &str) -> Page {
    Page::at(&["<body style=margin:0>", html].concat(), VP)
}

/* a<b>b</b>c is one word: no gap at the element boundaries. */
#[test]
fn element_boundaries_add_no_space() {
    let p = at("<p style=margin:0>a<b>b</b>c (<i>often</i>), N<b>X</b>NOS.</p>");
    let (a, b, c) = (p.word("a"), p.word("b"), p.word("c"));
    assert_eq!(b.x, a.x + a.w, "a|b");
    assert_eq!(c.x, b.x + b.w, "b|c");
    let (open, often, close) = (p.word("("), p.word("often"), p.word("),"));
    assert_eq!((often.x, close.x), (open.x + open.w, often.x + often.w), "(often),");
}

/* Spaces between elements collapse to one space's width. */
#[test]
fn a_space_between_elements_is_kept_once() {
    let p = at("<p style=margin:0><b>a</b>  <i> b</i></p>");
    let (a, b) = (p.word("a"), p.word("b"));
    let space = crate::browser::fonts::measure_text(0, false, false, " ", 16.0, 0.0);
    assert_eq!(b.x - (a.x + a.w), space);
}

/* Newlines between spans inside pre are line breaks: three lines. */
#[test]
fn newlines_between_spans_in_pre_break_lines() {
    let p = at(
        "<pre style=margin:0><span>r</span> <span>=</span>\n<span>s</span>\n<span>t</span></pre>",
    );
    let (r, eq, s, t) = (p.word("r"), p.word("="), p.word("s"), p.word("t"));
    assert_eq!(eq.y, r.y, "the preserved space keeps = on the first line");
    assert!(r.y < s.y && s.y < t.y, "three lines: {} {} {}", r.y, s.y, t.y);
    assert_eq!(eq.x - (r.x + r.w), r.w, "the space keeps its (monospace) width");
}

/* Punctuation after a link sits against it. */
#[test]
fn punctuation_after_a_link_sits_against_it() {
    let p = at("<p style=margin:0>In <a href=#>science</a>, a</p>");
    let (sci, comma) = (p.word("science"), p.word(","));
    assert_eq!(comma.x, sci.x + sci.w);
}

/* An underlined link's words make one run: each later word's fragment
 * takes the space before it, so the underline has no gaps. */
#[test]
fn a_link_underline_is_one_run() {
    let p = at("<p style=margin:0><a href=#>computer science</a> next</p>");
    let (a, b) = (p.word("computer"), p.word(" science"));
    assert_eq!(b.x, a.x + a.w, "the second fragment starts where the first ends");
    assert!(texts(&p.doc).iter().any(|t| t.3 == "next"), "unlinked text is not joined");
}

/* A line may break after a hyphen inside a word. */
#[test]
fn a_hyphenated_word_breaks_after_the_hyphen() {
    let p = at("<div style='width:60px'>well-known</div>");
    let (a, b) = (p.word("well-"), p.word("known"));
    assert!(b.y > a.y, "known wraps below well-");
}
