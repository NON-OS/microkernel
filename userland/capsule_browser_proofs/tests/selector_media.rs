// NONOS Operating System (AGPL-3.0-or-later)
//! Media Queries 4 range syntax and a unitless zero, on the fixed 1360 by
//! 760 viewport the engine lays out for. Both used to evaluate false, so a
//! mobile-only block showed on the desktop and (min-width: 0) never applied.

use capsule_browser_proofs::browser::css::parse::parse;

/* Whether the rule inside `@media <cond>` is kept. The parse also records
 * the condition's verdict as a rule with no selectors, which a resize
 * re-checks, so only a rule with selectors counts. */
fn applies(cond: &str) -> bool {
    parse(&format!("@media {cond} {{.a{{color:red}}}}")).iter().any(|r| !r.selectors.is_empty())
}

#[test]
fn range_comparisons_in_either_order() {
    assert!(applies("(width >= 40rem)"));
    assert!(!applies("(width < 40rem)"));
    assert!(applies("(1024px <= width)"));
    assert!(!applies("(1400px <= width)"));
    assert!(applies("(width > 1359px)"));
    assert!(!applies("(width > 1360px)"));
    assert!(applies("(width = 1360px)"));
    assert!(applies("(height <= 760px)"));
    assert!(!applies("(height < 760px)"));
}

#[test]
fn three_part_ranges_need_both_bounds() {
    assert!(applies("(1000px <= width < 1400px)"));
    assert!(!applies("(400px <= width <= 700px)"));
    assert!(applies("(1400px > width >= 1360px)"));
    assert!(!applies("(1000px <= width > 700px)"), "operators pointing apart are no range");
}

#[test]
fn aspect_ratio_and_unknown_features() {
    assert!(applies("(aspect-ratio > 16/9)"), "1360/760 is wider than 16:9");
    assert!(!applies("(aspect-ratio < 1)"));
    assert!(!applies("(resolution >= 2dppx)"), "an unknown range fails closed");
}

#[test]
fn a_unitless_zero_is_a_length() {
    assert!(applies("(min-width: 0)"));
    assert!(applies("(width >= 0)"));
    assert!(!applies("(min-width: 5)"), "other numbers still need a unit");
}
