// NONOS Operating System (AGPL-3.0-or-later)
//! Selector lists Chromium keeps, and so must the engine: :is() and
//! :where() forgive bad arguments, -webkit- pseudo-elements and the
//! pseudo-classes Chromium knows but NONOS can never match stay valid, and
//! escapes, comments and namespaces read as Selectors 4 reads them. Each
//! verdict here is Chromium's own for the same text.

use capsule_browser_proofs::browser::css::parse::parse_selectors;

#[test]
fn chromium_keeps_these_lists_and_so_does_the_engine() {
    for sel in [
        "p:is(!!)",
        "p:is()",
        "p:where(a,,b)",
        ".custom-range::-webkit-slider-thumb:active",
        "::-webkit-scrollbar:horizontal:hover",
        "p::-webkit-unknown",
        ".button::part(button):hover",
        "a::before::marker",
        "*, ::before, ::after",
        "p:-webkit-full-page-media",
        ":horizontal",
        "p:nth-child(2n + 1)",
        "p:nth-child(-n- 1)",
        "p:nth-child(99999999999)",
        "a:has(:is(:has(b)))",
        ":not(:is(::before))",
        "*|*",
        "|p",
        "[*|x]",
        "[x=a I]",
        "#-a",
        ".--a",
        "a/**/ b",
        ":hov\\65r",
        "p:dir(foo)",
        "& p",
    ] {
        assert!(!parse_selectors(sel).is_empty(), "{sel} must be valid");
    }
}
