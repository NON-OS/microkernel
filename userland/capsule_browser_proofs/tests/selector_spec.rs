// NONOS Operating System (AGPL-3.0-or-later)
//! Selectors 4 specificity for the 36 reference cases, 22 of which the
//! engine used to get wrong: pseudo-classes and :root count as classes,
//! pseudo-elements as types, :is/:not/:has as their most specific argument,
//! :where as nothing, nth-child(An+B of S) as a class plus S.

use capsule_browser_proofs::browser::css::parse::parse_selectors;
use capsule_browser_proofs::browser::css::specificity::specificity;

const CASES: &[(&str, (u32, u32, u32))] = &[
    ("*", (0, 0, 0)),
    ("li", (0, 0, 1)),
    ("ul li", (0, 0, 2)),
    ("ul ol+li", (0, 0, 3)),
    ("h1 + *[rel=up]", (0, 1, 1)),
    ("ul ol li.red", (0, 1, 3)),
    ("li.red.level", (0, 2, 1)),
    ("#x34y", (1, 0, 0)),
    ("#s12:not(FOO)", (1, 0, 1)),
    (".foo :is(.bar, #baz)", (1, 1, 0)),
    (":where(#a) .b", (0, 1, 0)),
    (":where(.a, .b) p", (0, 0, 1)),
    ("a:hover", (0, 1, 1)),
    ("a::before", (0, 0, 2)),
    ("a:before", (0, 0, 2)),
    ("li::marker", (0, 0, 2)),
    (":root", (0, 1, 0)),
    ("html:root", (0, 1, 1)),
    (":root.dark", (0, 2, 0)),
    (":not(.a)", (0, 1, 0)),
    (":not(.a, #b)", (1, 0, 0)),
    ("li:not(:first-child)", (0, 1, 1)),
    ("a:not([href])", (0, 1, 1)),
    ("li:first-child", (0, 1, 1)),
    ("li:nth-child(2n+1)", (0, 1, 1)),
    ("li:nth-child(2n+1 of .x)", (0, 2, 1)),
    ("[href]", (0, 1, 0)),
    ("a[href][title]", (0, 2, 1)),
    (":is(h1, .x)", (0, 1, 0)),
    (":is(h1, #x) p", (1, 0, 1)),
    (":has(> #a)", (1, 0, 0)),
    (".a .b .c .d .e .f .g .h .i .j .k", (0, 11, 0)),
    ("#a #b", (2, 0, 0)),
    ("p:empty", (0, 1, 1)),
    ("a:link", (0, 1, 1)),
    ("input:checked", (0, 1, 1)),
];

#[test]
fn every_reference_case_has_its_selectors_4_specificity() {
    for (sel, (a, b, c)) in CASES {
        let list = parse_selectors(sel);
        assert!(!list.is_empty(), "{sel} parses");
        /* An :is() of single compounds may split into one selector per
         * argument for the rule index; every copy keeps the :is() value. */
        for s in &list {
            let v = specificity(s);
            assert_eq!((v >> 20, (v >> 10) & 0x3ff, v & 0x3ff), (*a, *b, *c), "{sel}");
        }
    }
}
