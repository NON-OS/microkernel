use super::{fit, Recheck};
use crate::evm::tx::Eip1559;

const GWEI: u128 = 1_000_000_000;

fn deposit(value: u128, gas: u64) -> Eip1559 {
    let pool = [7; 20];
    let (max_priority_fee, max_fee) = (GWEI, 3 * GWEI);
    Eip1559 { chain_id: 1, nonce: 0, max_priority_fee, max_fee, gas, to: pool, value, data: vec![] }
}

// The review left exactly enough for its own gas: values plus limit times fee, to the wei.
fn reviewed() -> ([Eip1559; 2], u128) {
    let txs = [deposit(5 * GWEI, 100_000), deposit(2 * GWEI, 100_000)];
    (txs, 7 * GWEI + 2 * 100_000 * 3 * GWEI)
}

#[test]
fn the_review_as_it_stood_goes_out_unchanged() {
    let (txs, ether) = reviewed();
    assert_eq!(fit(ether, 3 * GWEI, &[100_000, 90_000], &txs), Recheck::Fits);
    assert_eq!(fit(ether - 1, 3 * GWEI, &[100_000, 100_000], &txs), Recheck::ShortOfEth);
}

#[test]
fn a_gas_rise_the_eth_left_cannot_pay_refuses_and_shrinks_nothing() {
    let (txs, ether) = reviewed();
    // One deposit now estimates one more unit of gas than it was signed for.
    assert_eq!(fit(ether, 3 * GWEI, &[100_001, 100_000], &txs), Recheck::ShortOfEth);
    // The fee ceiling rose by one wei.
    assert_eq!(fit(ether, 3 * GWEI + 1, &[100_000, 100_000], &txs), Recheck::ShortOfEth);
    // Paid for, but not what was signed: reviewed again, never sent.
    let more = ether + 100_000 * 3 * GWEI;
    assert_eq!(fit(more, 3 * GWEI, &[200_000, 100_000], &txs), Recheck::Moved);
    assert_eq!(fit(more, 4 * GWEI, &[100_000, 100_000], &txs), Recheck::Moved);
}

#[test]
fn a_hostile_answer_refuses() {
    let (txs, ether) = reviewed();
    assert_eq!(fit(ether, 3 * GWEI, &[100_000], &txs), Recheck::Moved);
    assert_eq!(fit(u128::MAX, u128::MAX, &[u64::MAX, u64::MAX], &txs), Recheck::ShortOfEth);
    assert_eq!(fit(0, 0, &[], &[]), Recheck::Fits);
}
