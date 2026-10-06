// NONOS Operating System (AGPL-3.0-or-later)
//! Combinators: the sibling ones (+ and ~) that used to be read as
//! descendants, whitespace that used to split brackets and parentheses, and
//! a descendant chain that has to backtrack to find its match. Expected
//! counts are Chromium's querySelectorAll on the same markup.

use capsule_browser_proofs::browser::{css, dom};

const PAGE: &str = "<html><head></head><body><h1>t</h1><p class=lead>a</p><p>b</p><ul class=list>\
<li class=first>1</li><li>2</li><li class=last>3</li></ul><div class=grid><span>s</span><em>e</em>\
<span>s</span><em>e</em></div><p data-mode='light dark'>c</p><table><tbody><tr><td>1</td><td>2\
</td></tr><tr><td>3</td><td>4</td></tr></tbody></table></body></html>";

fn count(html: &str, sel: &str) -> usize {
    css::select(&dom::parse(html.as_bytes()), sel, usize::MAX).len()
}

#[test]
fn sibling_combinators_match_like_chromium() {
    for (sel, want) in [
        ("h1 + p", 1),
        ("h1+p", 1),
        ("h1 ~ p", 3),
        ("h1~p", 3),
        ("li + li", 2),
        ("* + *", 15),
        (".grid > * + *", 3),
        ("span ~ em", 2),
        (".first ~ li", 2),
        ("h1 + p ~ p", 2),
        ("tr + tr > td", 2),
        (".lead ~ ul li", 3),
        ("li:first-child ~ li", 2),
    ] {
        assert_eq!(count(PAGE, sel), want, "{sel}");
    }
}

#[test]
fn whitespace_inside_brackets_and_parens_stays_inside() {
    assert_eq!(count(PAGE, "[data-mode='light dark']"), 1);
    assert_eq!(count(PAGE, "[ data-mode ]"), 1);
    assert_eq!(count(PAGE, "li:nth-child( 2n + 1 )"), 2);
    assert_eq!(count(PAGE, "[data-mode ^='light']"), 1);
    assert_eq!(count(PAGE, "[class~=lead]"), 1, "~= is an operator, not a combinator");
}

#[test]
fn descendant_chains_backtrack() {
    let page = "<html><body><div class=a><div class=b><div class=b><p class=c>x</p></div></div></div>\
<ul class=menu><li class=item><ul><li class=item><a class=lnk>x</a></li></ul></li></ul>\
<section class=s><div class=w><section class=s><div><span class=t>t</span></div></section></div></section>\
</body></html>";
    for sel in [
        ".a > .b .c",
        ".a > .b > .b .c",
        ".menu > .item a",
        ".menu > li .lnk",
        "body > div .c",
        ".s > .w .t",
        ".s > div .t",
        "section > div span",
    ] {
        assert_eq!(count(page, sel), 1, "{sel}");
    }
}
