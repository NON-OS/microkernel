// NONOS Operating System (AGPL-3.0-or-later)
//! The sibling table's shortcuts never change an answer: the ancestor
//! filter only rejects what cannot match, and hashed class names only speed
//! the same comparison. Every selector below is matched on every element
//! both with the table and by walking, and the two must agree.

use capsule_browser_proofs::browser::css::matching::{matches_selector, Siblings};
use capsule_browser_proofs::browser::css::parse::parse_selectors;
use capsule_browser_proofs::browser::dom;

const PAGE: &str = "<!DOCTYPE html><html class='js night'><head></head><body id=top class=page>\
<main class='content wide'><article class=post id=p1><h2 class=title>t</h2><p class=lead>a</p>\
<p class='x\ty\nz\u{c}w'>b</p><p class='u\u{a0}v'>c</p><ul><li class=item><a class=lnk>1</a></li>\
<li class='item last'><a class=lnk href=/2>2</a></li></ul></article><aside><div class=card>\
<div class=card><span class=deep>d</span></div></div></aside></main><footer><p>f</p></footer>\
</body></html>";

const SELECTORS: &[(&str, usize)] = &[
    (".page .lnk", 2),
    ("html.night main a", 2),
    (".js.night .post p", 3),
    ("#top .card .deep", 1),
    ("#p1 li + li a", 1),
    (".content > article .last a[href]", 1),
    ("body#top > main.wide aside span", 1),
    (".card .card .deep", 1),
    (".card > .card > .deep", 1),
    (".nothere .lnk", 0),
    ("#nope a", 0),
    ("footer p", 1),
    ("main p.y", 1),
    ("main p.z", 1),
    ("main p.w", 1),
    (".u\\a0 v", 1),
    (".u", 0),
    ("article :is(.lead, .title) ~ p", 3),
    (":not(.post) > ul a", 0),
    (".post:has(.last) .title", 1),
    ("li:nth-child(2) .lnk", 1),
    (".page :where(.x, .deep)", 2),
];

#[test]
fn table_and_walk_agree_everywhere() {
    let d = dom::parse(PAGE.as_bytes());
    let (table, walk) = (Siblings::table(&d), Siblings::walk());
    for &(text, want) in SELECTORS {
        let sels = parse_selectors(text);
        assert!(!sels.is_empty(), "{text} parses");
        let mut hits = 0;
        for id in 0..d.nodes.len() {
            let t = sels.iter().any(|s| matches_selector(&d, &table, id, s));
            let w = sels.iter().any(|s| matches_selector(&d, &walk, id, s));
            assert_eq!(t, w, "{text} at node {id}");
            hits += t as usize;
        }
        assert_eq!(hits, want, "{text} matches {want} elements");
    }
}

#[test]
fn class_values_split_on_ascii_whitespace_only() {
    let d = dom::parse(PAGE.as_bytes());
    let table = Siblings::table(&d);
    let count = |text: &str| {
        let sels = parse_selectors(text);
        (0..d.nodes.len())
            .filter(|&i| sels.iter().any(|s| matches_selector(&d, &table, i, s)))
            .count()
    };
    /* Tab, line feed and form feed separate class names; U+00A0 does not. */
    assert_eq!((count(".x"), count(".y"), count(".z"), count(".w")), (1, 1, 1, 1));
    assert_eq!((count(".u"), count(".v"), count(".u\\a0 v")), (0, 0, 1));
}
