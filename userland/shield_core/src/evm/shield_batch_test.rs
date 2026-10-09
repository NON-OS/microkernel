// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::panic, clippy::indexing_slicing)]

use super::{build, BatchChecked, Deposit};
use crate::evm::gas::Fees;
use crate::evm::Network;

const POOL: [u8; 20] = [7; 20];
const FEES: Fees = Fees { max_priority_fee: 1_000_000_000, max_fee: 3_000_000_000 };

fn two() -> [Deposit<'static>; 2] {
    [Deposit { value: 200, data: b"first" }, Deposit { value: 100, data: b"second" }]
}

/// Each deposit gets the next nonce, the shared fees and its own value and call.
#[test]
fn the_deposits_go_out_in_order_on_consecutive_nonces() {
    let chain = Network::Sepolia.chain();
    let gas = [180_000, 190_000];
    let ether = 300 + 370_000 * 3_000_000_000;
    let BatchChecked::Ready(batch) = build(&chain, (&POOL, &two(), &gas), (41, FEES), ether)
        .unwrap_or(BatchChecked::Refused("error"))
    else {
        panic!("the balance covers the batch");
    };
    assert_eq!(batch.max_network_fee, 370_000 * 3_000_000_000);
    assert_eq!(batch.txs.iter().map(|t| t.nonce).collect::<Vec<_>>(), vec![41, 42]);
    assert_eq!((batch.txs[0].value, batch.txs[1].data.as_slice()), (200, b"second".as_slice()));
    assert!(batch.txs.iter().all(|t| t.to == POOL && t.chain_id == 11_155_111));
}

/// The amounts and the most every network fee can be are one sum, and one wei short refuses all.
#[test]
fn a_balance_one_wei_short_of_the_whole_refuses_every_deposit() {
    let chain = Network::Sepolia.chain();
    let gas = [180_000, 190_000];
    let ether = 300 + 370_000 * 3_000_000_000 - 1;
    let short = build(&chain, (&POOL, &two(), &gas), (41, FEES), ether);
    assert!(matches!(short, Ok(BatchChecked::Refused(_))));
}

/// A nonce at the top of the range is refused as a malformed reply, never a panic.
#[test]
fn a_nonce_past_the_range_is_refused_not_panicked() {
    let chain = Network::Sepolia.chain();
    let top = build(&chain, (&POOL, &two(), &[1, 1]), (u64::MAX, FEES), u128::MAX);
    assert!(top.is_err());
}
