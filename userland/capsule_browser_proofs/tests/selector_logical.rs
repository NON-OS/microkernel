// NONOS Operating System (AGPL-3.0-or-later)
//! :is(), :where(), :-webkit-any(), :not() and :has() take real selector
//! lists: complex arguments with their own combinators, any number of them,
//! matched with the element as their subject. Text expansion used to cut
//! them at 8 alternatives and splice complex arguments into the outer
//! compound. Expected counts are Chromium's on this markup.

use capsule_browser_proofs::browser::{css, dom};

const PAGE: &str = "<!DOCTYPE html><html><head></head><body><main><h1>T</h1><p class=lead>L</p>\
<p class=note> </p><p class=empty></p><h2>S</h2><p>A</p><ol class=steps><li class=step>a</li>\
<li class='step done'>b</li><li>x</li><li class=step>c</li></ol><div class=grid><span>1</span>\
<em>2</em><span>3</span><em>4</em><b>5</b></div><div class=card><div class=body><p class=text>t</p>\
<p class='text muted'>m</p></div></div><div class=dark><div class=card>h</div></div><nav>\
<a class=x>n</a></nav><p><a class=x>o</a></p><ul><li><a href=/1>1</a></li><li><span>2</span>\
</li></ul></main></body></html>";

fn check(cases: &[(&str, usize)]) {
    let d = dom::parse(PAGE.as_bytes());
    for (sel, want) in cases {
        assert_eq!(css::select(&d, sel, usize::MAX).len(), *want, "{sel}");
    }
}

#[test]
fn matches_any_takes_complex_arguments_without_a_cap() {
    check(&[
        (":is(h1, h2) + p", 2),
        ("p:is(.card p)", 2),
        ("p:is(main > p)", 5),
        (".text:is(.card .body *)", 2),
        (":is(.a1,.a2,.a3,.a4,.a5,.a6,.a7,.a8,.a9,.lead)", 1),
        (":-webkit-any(h1, h2)", 2),
        (".card:where(.dark,.dark *)", 1),
        (".x:is(nav a)", 1),
    ]);
}

#[test]
fn not_takes_lists_and_complex_arguments() {
    check(&[("p:not(.card p)", 5), ("p:not(main > p)", 2), ("li:not(.step, .done)", 3)]);
}

#[test]
fn has_looks_down_and_across_from_its_subject() {
    check(&[
        (".card:has(.muted)", 1),
        ("li:has(> a[href])", 1),
        ("h1:has(+ p)", 1),
        ("div:has(p)", 2),
        (":not(:has(p))", 31),
        ("main :has(> span)", 2),
    ]);
}

#[test]
fn nth_child_of_counts_among_the_matching_siblings() {
    check(&[
        ("li:nth-child(2 of .step)", 1),
        ("li:nth-child(odd of .step)", 2),
        ("li:nth-last-child(1 of .step)", 1),
    ]);
}
