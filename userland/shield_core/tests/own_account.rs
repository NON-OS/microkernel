/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used)]

//! A private transfer to an account of the same wallet is recognised before it is sent.

mod keystore;
mod restored;

use keystore::TestGuard;
use nox_shield_core::ffi::Wallet;
use restored::restored;
use std::sync::Arc;

#[test]
fn a_transfer_to_an_own_account_is_recognised() {
    let dir = std::env::temp_dir().join(format!("nox-own-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let wallet = restored(&dir);
    wallet.add_account().expect("a second account");
    let second = wallet.receiving_address().expect("nox1");
    wallet.select_account(0).expect("back to the first");
    assert_eq!(wallet.own_account_of(second).expect("parsed"), Some(1));
    let other = std::env::temp_dir().join(format!("nox-own-other-{}", std::process::id()));
    std::fs::create_dir_all(&other).expect("a temporary directory");
    let stranger =
        Wallet::new(other.to_string_lossy().into_owned(), Arc::new(TestGuard)).expect("a wallet");
    stranger.create().expect("another wallet");
    let theirs = stranger.receiving_address().expect("nox1");
    assert_eq!(wallet.own_account_of(theirs).expect("parsed"), None);
    assert!(!wallet.arrived_recently().expect("no sync yet, so nothing recent"));
    stranger.wipe().expect("the wipe");
    wallet.wipe().expect("the wipe");
    std::fs::remove_dir_all(&dir).ok();
    std::fs::remove_dir_all(&other).ok();
}
