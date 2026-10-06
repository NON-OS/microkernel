// NONOS Operating System (AGPL-3.0-or-later)
//! A selector list is valid exactly when Chromium keeps it: one invalid
//! selector drops the whole rule, as do unknown or vendor pseudo-classes
//! Chromium does not know, a pseudo-class after ::before, and malformed
//! arguments. Each verdict here is Chromium's own for the same text; the
//! lists it keeps are in selector_validity_kept.rs.

use capsule_browser_proofs::browser::css::parse::parse_selectors;

fn valid(sel: &str) -> bool {
    !parse_selectors(sel).is_empty()
}

#[test]
fn chromium_drops_these_lists_and_so_does_the_engine() {
    for sel in [
        "p, :unknown-pseudo",
        "p, ::unknown",
        "p,",
        ",p",
        "p,,a",
        "h1 h2 >",
        "> p",
        "a > > b",
        "a || b",
        "h1, !!, p",
        "button:-moz-focusring",
        "button::-moz-focus-inner, input",
        "body:not(:-moz-handler-blocked) fieldset",
        "_::-webkit-full-page-media, ol.references > li",
        ".gl-toggle::before::selection",
        "a::before:hover",
        "a::before .x",
        "p:matches(a)",
        "p:not(::before)",
        "p:has(:has(a))",
        "p:has()",
        "p:not()",
        "p:nth-child(2 n+1)",
        "p:nth-of-type(2 of .a)",
        "p:lang(\"en\")",
        "#1a",
        ".1a",
        "p*",
        "&p",
        "[x==a]",
    ] {
        assert!(!valid(sel), "{sel} must be invalid");
    }
}
