/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used)]

//! A withdrawal pays an unused address of this wallet: the next account's, which moves on when an
//! account is added. A wallet from a private key has no next account to offer.

mod keystore;
mod restored;

use keystore::TestGuard;
use nox_shield_core::ffi::Wallet;
use restored::restored;
use std::sync::Arc;

#[test]
fn the_fresh_address_is_the_next_account_and_moves_on() {
    let dir = std::env::temp_dir().join(format!("nox-fresh-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let wallet = restored(&dir);
    let fresh = wallet.fresh_withdrawal_address().expect("read");
    assert_eq!(fresh.as_deref(), Some("0x6Fac4D18c912343BF86fa7049364Dd4E424Ab9C0"));
    wallet.add_account().expect("account 2");
    let next = wallet.fresh_withdrawal_address().expect("read");
    assert_eq!(next.as_deref(), Some("0xb6716976A3ebe8D39aCEB04372f22Ff8e6802D7A"));
    wallet.lock().expect("locked");
    wallet.unlock().expect("unlocked");
    assert_eq!(wallet.fresh_withdrawal_address().expect("read"), next, "the same after a lock");
    wallet.wipe().expect("the wipe");

    let key_dir = std::env::temp_dir().join(format!("nox-fresh-key-{}", std::process::id()));
    std::fs::create_dir_all(&key_dir).expect("a temporary directory");
    let key_wallet =
        Wallet::new(key_dir.to_string_lossy().into_owned(), Arc::new(TestGuard)).expect("a wallet");
    key_wallet
        .restore_from_key(
            "0x1ab42cc412b618bdea3a599e3c9bae199ebf030895b039e9db1e30dafb12b727".into(),
        )
        .expect("restored");
    assert_eq!(key_wallet.fresh_withdrawal_address().expect("read"), None);
    key_wallet.wipe().expect("the wipe");
    std::fs::remove_dir_all(&dir).ok();
    std::fs::remove_dir_all(&key_dir).ok();
}
