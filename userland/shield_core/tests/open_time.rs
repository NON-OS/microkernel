//! Opening a wallet from its words is quick: a restore and its private address in well under
//! the time the NONOS wallet allows an open, so an open that runs for minutes on a machine is
//! stuck, not slow. On this host in release a restore takes some 17 ms, and some 170 ms under
//! QEMU's TCG emulation; the bound here leaves room for a debug build on a slow runner.

mod keystore;

use keystore::{scratch, TestGuard};
use nox_shield_core::ffi::Wallet;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Far under the 60 seconds after which the NONOS wallet says an open is stuck.
const BOUND: Duration = Duration::from_secs(5);

fn words() -> Vec<String> {
    let mut words = vec!["abandon".to_string(); 11];
    words.push("about".to_string());
    words
}

#[test]
fn restoring_from_the_words_shows_the_private_address_within_the_bound() {
    let dir = scratch("open-time-restore");
    let wallet = Wallet::new(dir.to_string_lossy().into_owned(), Arc::new(TestGuard))
        .expect("a wallet under a temporary directory");
    let began = Instant::now();
    wallet.restore(words()).expect("the standard phrase");
    let address = wallet.receiving_address().expect("the private address");
    let took = began.elapsed();
    assert!(address.starts_with("nox1"), "a private address, not {address}");
    assert!(took < BOUND, "restore to first address took {took:?}");
}

#[test]
fn unlocking_a_stored_wallet_shows_the_private_address_within_the_bound() {
    let dir = scratch("open-time-unlock");
    let wallet = Wallet::new(dir.to_string_lossy().into_owned(), Arc::new(TestGuard))
        .expect("a wallet under a temporary directory");
    wallet.restore(words()).expect("the standard phrase");
    let first = wallet.receiving_address().expect("the private address");
    wallet.lock().expect("lock");
    let began = Instant::now();
    wallet.unlock().expect("unlock");
    let again = wallet.receiving_address().expect("the private address");
    let took = began.elapsed();
    assert_eq!(first, again, "the same words open the same address");
    assert!(took < BOUND, "unlock to first address took {took:?}");
}
