// NONOS Operating System (AGPL-3.0-or-later)
//! Links, language, direction, definedness and open state from the tree,
//! and the states no NONOS document can be in, which stay valid and match
//! nothing. Expected counts are Chromium's on this markup.

use capsule_browser_proofs::browser::{css, dom};

const PAGE: &str = "<!DOCTYPE html><html lang=en><head><link rel=stylesheet href=x.css></head>\
<body><a href=/a class=l>a</a><a class=n>b</a><area href=/c><section lang=fr-CA><p>b<span>c</span>\
</p></section><section lang=de-CH><p>h</p></section><div lang=''><p>u</p></div><div dir=rtl><p>r</p>\
</div><div dir=auto><p>\u{5e9}\u{5dc}\u{5d5}\u{5dd}</p></div><bdi>abc</bdi><custom-el>x</custom-el>\
<font-face>f</font-face><details open><summary>s</summary></details><details><summary>t</summary>\
</details><dialog open>d</dialog><p id=t>target</p></body></html>";

fn check(cases: &[(&str, usize)]) {
    let d = dom::parse(PAGE.as_bytes());
    for (sel, want) in cases {
        assert_eq!(css::select(&d, sel, usize::MAX).len(), *want, "{sel}");
    }
}

#[test]
fn links_are_a_and_area_with_an_href_and_none_is_visited() {
    check(&[(":link", 2), (":any-link", 2), ("a:link", 1), (":-webkit-any-link", 2)]);
    check(&[(":visited", 0)]);
}

#[test]
fn language_inherits_and_filters_by_subtag() {
    check(&[(":lang(fr)", 3), (":lang(en)", 20), (":lang(de)", 2), (r":lang(\*-CH)", 2)]);
    check(&[(":lang(fr-CA)", 3), ("p:lang(en)", 3)]);
}

#[test]
fn direction_inherits_and_auto_reads_the_first_strong_letter() {
    check(&[(":dir(rtl)", 4), (":dir(ltr)", 23), ("p:dir(rtl)", 2)]);
}

#[test]
fn definedness_and_open_state() {
    check(&[(":defined", 26), (":not(:defined)", 1), (":open", 2), ("details:open", 1)]);
}

#[test]
fn states_a_nonos_document_cannot_be_in_are_valid_and_match_nothing() {
    for sel in [":target", ":hover", ":focus", ":focus-within", ":active", ":fullscreen"] {
        check(&[(sel, 0)]);
    }
    for sel in [":autofill", ":modal", ":popover-open", ":user-invalid", ":host", ":past"] {
        check(&[(sel, 0)]);
        assert!(!css::parse::parse_selectors(sel).is_empty(), "{sel} is valid");
    }
}
