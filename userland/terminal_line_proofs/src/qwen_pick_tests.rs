// NONOS Operating System (AGPL-3.0-or-later)
/* The tier `qwen` runs with none named: the one policy holds, if it is a tier. */

use super::ask::parse;
use super::tiers::{pick, TIERS};

#[test]
fn a_known_tier_is_taken() {
    assert_eq!(pick(b"qwen3-1.7b"), b"qwen3-1.7b");
    assert_eq!(pick(b"coder-7b"), b"coder-7b");
}

#[test]
fn nothing_or_an_unknown_word_is_the_first_tier() {
    assert_eq!(pick(b""), TIERS[0]);
    assert_eq!(pick(b"qwen3-1.7"), TIERS[0]);
    assert_eq!(pick(b"QWEN3-1.7B"), TIERS[0]);
}

#[test]
fn a_typed_tier_still_wins() {
    let a = parse(b"qwen large what is 2+2").unwrap();
    assert_eq!(a.tier(), b"large");
    assert_eq!(parse(b"qwen what is 2+2").unwrap().tier(), TIERS[0]);
}
