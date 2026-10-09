/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used)]

//! The account search after a restore, against the live pool and both networks over Tor. A new
//! phrase has never been used, so the search stops after five empty accounts and keeps one.

mod keystore;

use keystore::TestGuard;
use nox_shield_core::ffi::Wallet;
use std::sync::Arc;

#[test]
#[ignore]
fn a_new_phrase_restores_to_one_account() {
    let dir = std::env::temp_dir().join(format!("nox-find-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let path = dir.to_string_lossy().into_owned();
    let first = Wallet::new(path.clone(), Arc::new(TestGuard)).expect("a wallet");
    let words = first.create().expect("a new phrase");
    first.wipe().expect("the wipe");
    let wallet = Wallet::new(path, Arc::new(TestGuard)).expect("a wallet");
    wallet.restore(words).expect("the restore");
    let warm = std::time::Instant::now();
    wallet.public_balances(nox_shield_core::evm::Network::Sepolia).expect("Tor started");
    println!("Tor started and one read in {:?}", warm.elapsed());
    let started = std::time::Instant::now();
    assert_eq!(wallet.find_accounts().expect("the search"), 1);
    println!("the search took {:?}", started.elapsed());
    assert_eq!(wallet.account_search_progress(), 5, "five accounts looked at, then the gap");
    assert!(started.elapsed().as_secs() < 120, "the search took {:?}", started.elapsed());
    assert_eq!(wallet.find_accounts().expect("nothing left to search"), 1);
    wallet.wipe().expect("the wipe");
    std::fs::remove_dir_all(&dir).ok();
}
