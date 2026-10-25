//! AmountPolicy.rules(): four words read strictly, and the pool's _standard mirrored for the tests.

use nox_shield_core::net::policy_rules::{decode_rule, Rule, MAX_EXP};

fn word(v: u64) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[24..].copy_from_slice(&v.to_be_bytes());
    w
}

fn reply(fee: u64, min: u64, max: u64, ranged: u64) -> Vec<u8> {
    [word(fee), word(min), word(max), word(ranged)].concat()
}

#[test]
fn a_rule_reads_strictly() {
    let r = decode_rule(&reply(5, 3, 6, 1)).unwrap();
    assert_eq!(r, Rule { fee: 5, min_exp: 3, max_exp: 6, ranged: true });
    assert_eq!(r.range(), (1_000, 5_000_000));
    assert!(decode_rule(&reply(0, 0, u64::from(MAX_EXP), 0)).is_some());
    for (bad, why) in [
        (reply(5, 3, 20, 1), "an exponent past MAX_EXP"),
        (reply(5, 7, 6, 1), "minExp above maxExp"),
        (reply(5, 3, 6, 2), "a bool of 2"),
        (reply(5, 256, 300, 1), "an exponent past a byte"),
        (reply(5, 3, 6, 1)[..96].to_vec(), "three words"),
    ] {
        assert_eq!(decode_rule(&bad), None, "{why}");
    }
    let mut wide = reply(5, 3, 6, 1);
    wide[23] = 1;
    assert_eq!(decode_rule(&wide), None, "a fee past u64");
}

#[test]
fn standard_is_one_two_or_five_times_ten_to_k_inside_the_exponents() {
    let r = Rule { fee: 0, min_exp: 3, max_exp: 6, ranged: true };
    for ok in [1_000, 2_000, 5_000, 10_000, 5_000_000] {
        assert!(r.standard(ok), "{ok}");
    }
    for no in [0, 100, 500, 3_000, 1_500, 10_000_000, 50_000_000] {
        assert!(!r.standard(no), "{no}");
    }
    assert!(!Rule { ranged: false, ..r }.standard(1_000), "not ranged: nothing is standard");
    let top = Rule { fee: 0, min_exp: 0, max_exp: MAX_EXP, ranged: true };
    assert!(top.standard(10_000_000_000_000_000_000) && top.standard(1));
}
