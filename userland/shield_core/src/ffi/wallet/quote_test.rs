// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used)]

use super::{Quoted, Wallet};
use crate::custody::HardwareGuard;
use crate::error::{CustodyError, WalletError};
use crate::net::asset::Coin;
use std::sync::Arc;

struct Open;

impl HardwareGuard for Open {
    fn wrap(&self, key: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        Ok(key)
    }

    fn unwrap_key(&self, blob: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        Ok(blob)
    }
}

fn holding(order: (Coin, u64, bool), fee: u64) -> Arc<Wallet> {
    let dir = std::env::temp_dir().join("nox-quote-test");
    let wallet = Wallet::new(dir.to_string_lossy().into_owned(), Arc::new(Open)).expect("a wallet");
    *wallet.quoted.lock().expect("the slot") = Some(Quoted { order, fee });
    wallet
}

/// The quote binds the fee of the order it was made for, once, and no other order.
#[test]
fn a_spend_proves_the_fee_it_was_quoted_or_none() {
    let order = (Coin::Eth, 10_000_000_000_000_000, false);
    let same = holding(order, 3_000_000_000_000_000);
    assert_eq!(same.keep_to_quote(order, 3_000_000_000_000_000), Ok(()));
    assert_eq!(same.keep_to_quote(order, 5_500_000_000_000_000), Ok(()), "a quote is used once");
    let moved = holding(order, 3_000_000_000_000_000);
    let refused = moved.keep_to_quote(order, 5_500_000_000_000_000);
    assert_eq!(refused, Err(WalletError::FeeChanged));
    let other = holding((Coin::Nox, 1_000_000_000_000, true), 2_400_000_000_000);
    assert_eq!(other.keep_to_quote(order, 5_500_000_000_000_000), Ok(()));
}
