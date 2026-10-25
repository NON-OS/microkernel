use super::{pieces, pieces_by, MAX_PIECES};
use crate::net::asset_v2::{PROD_ETH, PROD_NOX};
use crate::net::policy_rules::Rule;

const CENT: u64 = 10_000_000_000_000_000;
const THOUSAND_NOX: u64 = 1_000_000_000_000;
const WIDE_CENT: u128 = CENT as u128;

#[test]
fn a_typed_amount_becomes_the_fewest_standard_deposits() {
    assert_eq!(
        pieces(37 * WIDE_CENT, &PROD_ETH),
        Some(vec![20 * CENT, 10 * CENT, 5 * CENT, 2 * CENT])
    );
    assert_eq!(pieces(WIDE_CENT, &PROD_ETH), Some(vec![CENT]));
    assert_eq!(
        pieces(5000 * WIDE_CENT, &PROD_ETH),
        Some(vec![1000 * CENT; 5]),
        "10 ETH is the largest"
    );
    let nox = pieces(1234 * u128::from(THOUSAND_NOX), &PROD_NOX);
    let k = THOUSAND_NOX;
    assert_eq!(nox, Some(vec![1000 * k, 200 * k, 20 * k, 10 * k, 2 * k, 2 * k]));
}

#[test]
fn an_amount_the_sizes_cannot_make_is_refused_whole() {
    assert_eq!(pieces(WIDE_CENT + WIDE_CENT / 2, &PROD_ETH), None, "half a cent is left over");
    assert_eq!(pieces(WIDE_CENT - 1, &PROD_ETH), None);
    assert_eq!(pieces(0, &PROD_ETH), None);
    assert_eq!(pieces(u128::from(THOUSAND_NOX / 2), &PROD_NOX), None);
    assert_eq!(pieces(21_000 * WIDE_CENT, &PROD_ETH), None, "21 deposits of 10 ETH pass the cap");
}

/// Every whole number of cents to 100 ETH: the deposits add up, each is a size the pool takes,
/// and no digit costs more than three.
#[test]
fn every_amount_in_cents_to_one_hundred_eth_splits_into_allowed_sizes() {
    for cents in 1..=10_000u64 {
        let made = pieces(u128::from(cents) * WIDE_CENT, &PROD_ETH).unwrap_or_default();
        let sum: u128 = made.iter().map(|p| u128::from(*p)).sum();
        assert_eq!(sum, u128::from(cents) * WIDE_CENT, "{cents} cents");
        assert!(made.iter().all(|p| PROD_ETH.allows(*p)), "{cents} cents");
        let digits = format!("{cents}").len() as u64;
        let tens = cents / 1000;
        assert!(made.len() as u64 <= 3 * digits.min(4) + tens, "{cents} cents in {made:?}");
        assert!(made.len() <= MAX_PIECES);
    }
}

/// With rules() read, the split follows the policy's range, not the asset's own.
#[test]
fn a_rule_read_from_the_policy_sets_the_sizes() {
    let rule = Rule { fee: 0, min_exp: 2, max_exp: 3, ranged: true };
    let by = |u: u128| pieces_by(u, |s| rule.standard(s));
    assert_eq!(by(3_700), Some(vec![2_000, 1_000, 500, 200]));
    assert_eq!(by(12_000), Some(vec![5_000, 5_000, 2_000]), "5 x 10^maxExp is the largest");
    assert_eq!(by(150), None, "50 is under 10^minExp");
    let off = Rule { ranged: false, ..rule };
    assert_eq!(pieces_by(3_700, |s| off.standard(s)), None, "not ranged: nothing is standard");
}
