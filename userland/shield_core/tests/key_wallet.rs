/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used)]

//! A wallet restored from a private key: its public account is that key, it has one account and
//! no words, its key comes back on export, and its shield is its own. The key is the public one of
//! the test phrase at index 0.

mod keystore;
mod restored;

use keystore::TestGuard;
use nox_shield_core::error::{CustodyError, WalletError};
use nox_shield_core::ffi::Wallet;
use restored::restored;
use std::sync::Arc;

const KEY: &str = "0x1ab42cc412b618bdea3a599e3c9bae199ebf030895b039e9db1e30dafb12b727";
const ADDRESS: &str = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";

#[test]
fn a_private_key_restores_its_account_and_a_shield_of_its_own() {
    let dir = std::env::temp_dir().join(format!("nox-key-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let wallet =
        Wallet::new(dir.to_string_lossy().into_owned(), Arc::new(TestGuard)).expect("a wallet");
    let bad = wallet.restore_from_key("0x1234".into());
    assert_eq!(bad, Err(WalletError::Custody { source: CustodyError::KeyShape }));
    wallet.restore_from_key(KEY.into()).expect("restored");
    assert_eq!(wallet.public_address().expect("the address"), ADDRESS);
    assert!(wallet.from_key().expect("known"));
    wallet.lock().expect("locked");
    wallet.unlock().expect("unlocked");
    assert_eq!(wallet.public_address().expect("the same after a lock"), ADDRESS);
    assert_eq!(wallet.export_public_key(0).expect("the key"), KEY);
    assert_eq!(wallet.add_account(), Err(WalletError::Unavailable));
    assert_eq!(wallet.recovery_words().expect("opened"), None);
    let from_key = wallet.receiving_address().expect("nox1");
    wallet.wipe().expect("the wipe");

    let words_dir = std::env::temp_dir().join(format!("nox-key-words-{}", std::process::id()));
    std::fs::create_dir_all(&words_dir).expect("a temporary directory");
    let from_words = restored(&words_dir);
    assert_eq!(from_words.public_address().expect("the address"), ADDRESS);
    assert_ne!(from_words.receiving_address().expect("nox1"), from_key, "the shields are apart");
    from_words.wipe().expect("the wipe");
    std::fs::remove_dir_all(&dir).ok();
    std::fs::remove_dir_all(&words_dir).ok();
}
