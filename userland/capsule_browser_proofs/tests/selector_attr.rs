// NONOS Operating System (AGPL-3.0-or-later)
//! Attribute selectors: quoted values keep their spaces, brackets and
//! commas; a trailing i flag compares ASCII case-insensitively and s
//! exactly; namespace prefixes are read; and without a flag a value still
//! compares exactly, as the older attribute proofs pin.

use capsule_browser_proofs::browser::{css, dom};

const PAGE: &str =
    "<!DOCTYPE html><html><head></head><body><div class=card data-kind='Primary Card' \
data-x='a b' data-y='a]b' data-z='a,b' title='Hello World'>c</div><div data-kind=secondary>d</div>\
<input type=SUBMIT><svg><circle r=1></circle></svg></body></html>";

fn n(sel: &str) -> usize {
    css::select(&dom::parse(PAGE.as_bytes()), sel, usize::MAX).len()
}

#[test]
fn quoted_values_keep_what_the_quotes_hold() {
    assert_eq!(n("[data-kind=\"Primary Card\"]"), 1);
    assert_eq!(n("[data-x='a b']"), 1);
    assert_eq!(n("[data-y=\"a]b\"]"), 1);
    assert_eq!(n("[data-z='a,b'], [title='Hello World']"), 1);
}

#[test]
fn the_i_and_s_flags() {
    assert_eq!(n("[data-kind=\"primary card\" i]"), 1);
    assert_eq!(n("[data-kind=\"PRIMARY CARD\" I]"), 1);
    assert_eq!(n("[data-kind^=\"PRIM\" i]"), 1);
    assert_eq!(n("[data-kind=\"primary card\"]"), 0);
    assert_eq!(n("[data-kind=\"secondary\" s]"), 1);
    assert_eq!(n("input[type=\"submit\" i]"), 1);
    assert_eq!(n("input[type=submit]"), 0, "no implicit HTML case folding yet");
}

#[test]
fn namespace_prefixes() {
    assert_eq!(n("*|circle"), 1);
    assert_eq!(n("*|*"), n("*"));
    assert_eq!(n("|circle"), 0, "no element of an HTML document lacks a namespace");
    assert_eq!(n("[*|data-kind]"), 2);
    assert_eq!(n("[|data-kind]"), 2);
}
