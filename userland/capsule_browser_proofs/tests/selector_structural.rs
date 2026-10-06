// NONOS Operating System (AGPL-3.0-or-later)
//! Positional pseudo-classes the engine used to drop as never-matching
//! (nth-last-child, the of-type family), :empty with whitespace (content,
//! as Chromium counts it) and :scope. Expected counts are Chromium's.

use capsule_browser_proofs::browser::{css, dom};

const PAGE: &str = "<!DOCTYPE html><html><head></head><body><main><h1>T</h1><p class=lead>L</p>\
<p class=note> </p><p class=empty></p><ol><li class=step>a</li><li class='step done'>b</li>\
<li>x</li><li class=step>c</li></ol><div class=grid><span>1</span><em>2</em><span>3</span>\
<em>4</em><b>5</b></div></main></body></html>";

fn check(cases: &[(&str, usize)]) {
    let d = dom::parse(PAGE.as_bytes());
    for (sel, want) in cases {
        let table = css::select(&d, sel, usize::MAX);
        assert_eq!(table.len(), *want, "{sel}");
        /* The per-node walk element.matches() uses agrees with the table. */
        let walk: Vec<usize> = (0..d.nodes.len()).filter(|&i| css::matches(&d, i, sel)).collect();
        assert_eq!(table, walk, "{sel}");
    }
}

#[test]
fn the_nth_and_of_type_families_count_positions() {
    check(&[
        ("li:nth-last-child(2)", 1),
        ("li:nth-child(-n+2)", 2),
        ("li:nth-child(n+3)", 2),
        (".grid span:nth-of-type(2)", 1),
        (".grid em:nth-last-of-type(1)", 1),
        (".grid b:only-of-type", 1),
        ("p:only-of-type", 0),
        ("h1:only-of-type", 1),
    ]);
}

#[test]
fn empty_means_no_elements_and_no_text_at_all() {
    check(&[("p:empty", 1), (".note:empty", 0)]);
}

#[test]
fn scope_is_the_query_root() {
    let d = dom::parse(PAGE.as_bytes());
    for (sel, want) in [(":scope", 1), (":scope > body", 1), ("& main", 1)] {
        assert_eq!(css::select(&d, sel, usize::MAX).len(), want, "document query: {sel}");
    }
    /* element.matches(':scope') asks about the element itself. */
    let li = css::select(&d, "li", 1)[0];
    assert!(css::matches(&d, li, ":scope") && css::matches(&d, li, "ol > :scope"));
}
