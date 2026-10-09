// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::indexing_slicing)]

use super::{plan, refused};
use crate::custody::Seed;
use crate::keys::Account;
use crate::wallet::prepare_deposit;

/// 0.3 ETH as 0.2 and 0.1 at the 0.50% deposit fee of the production policy.
#[test]
fn a_split_shows_each_deposit_and_the_whole() {
    let to = Account::from_seed(&Seed::new([7u8; 64])).expect("an account").address();
    let a = prepare_deposit(&to, 200_000_000_000_000_000, 0, 50, 1).expect("a deposit");
    let b = prepare_deposit(&to, 100_000_000_000_000_000, 0, 50, 1).expect("a deposit");
    let view = plan(9, &[a, b], 1, 371_000_000_000_000);
    assert_eq!(view.pieces.len(), 2);
    let first = &view.pieces[0];
    assert_eq!((first.amount.as_str(), first.pool_fee.as_str()), ("0.2", "0.001"));
    assert_eq!(first.shielded, "0.199");
    assert_eq!((view.amount.as_str(), view.pool_fee.as_str()), ("0.3", "0.0015"));
    assert_eq!((view.shielded.as_str(), view.max_network_fee.as_str()), ("0.2985", "0.000371"));
    assert_eq!((view.id, view.approval, view.valid_for_seconds), (9, false, 90));
}

#[test]
fn a_refused_split_holds_nothing_to_confirm() {
    let view = refused("why");
    assert_eq!((view.id, view.pieces.len(), view.valid_for_seconds), (0, 0, 0));
    assert_eq!(view.refusal.as_deref(), Some("why"));
}
