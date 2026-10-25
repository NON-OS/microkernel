// NONOS Operating System (AGPL-3.0-or-later)
//! A release kernel refuses a trust set or a policy key named at build time;
//! any other build may take either.

use crate::build_baked_trust::{refusal, TEST_TRUST};

const DIR: &str = "NONOS_BAKED_TRUST_DIR";
const KEY: &str = "NONOS_DEVICE_POLICY_PUBKEY";

fn named(dir: bool, key: bool) -> [(&'static str, bool); 2] {
    [(DIR, dir), (KEY, key)]
}

#[test]
fn both_overrides_are_the_ones_held() {
    assert_eq!(TEST_TRUST, [DIR, KEY]);
}

#[test]
fn a_release_build_refuses_each_override_and_names_it() {
    let dir = refusal(true, &named(true, false)).expect("a test set in a release build");
    assert!(dir.contains(DIR) && !dir.contains(KEY), "{dir}");
    let key = refusal(true, &named(false, true)).expect("a test key in a release build");
    assert!(key.contains(KEY) && !key.contains(DIR), "{key}");
    let both = refusal(true, &named(true, true)).expect("both in a release build");
    assert!(both.contains(DIR) && both.contains(KEY), "{both}");
}

#[test]
fn a_release_build_with_the_trees_trust_goes_on() {
    assert_eq!(refusal(true, &named(false, false)), None);
}

#[test]
fn a_build_that_is_not_for_release_may_take_a_test_set() {
    for (d, k) in [(false, false), (true, false), (false, true), (true, true)] {
        assert_eq!(refusal(false, &named(d, k)), None);
    }
}

#[test]
fn the_build_asks_before_it_reads_the_set() {
    let src = std::fs::read_to_string("../../build_baked_trust.rs").unwrap();
    let decl = &src[src.find("pub fn declare_baked_trust").expect("declare")..];
    let at = |s: &str| decl.find(s).unwrap_or_else(|| panic!("no {s}"));
    assert!(at("CARGO_FEATURE_NONOS_RELEASE") < at("refusal(release, &named)"));
    assert!(at("refusal(release, &named)") < at("panic!(\"{why}\")"));
    assert!(at("panic!(\"{why}\")") < at("let manifest ="));
}
