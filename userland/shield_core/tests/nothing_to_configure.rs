/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used, clippy::panic)]

//! A wallet needs nothing configured to exist.
//!
//! First run used to demand a sequencer onion before a wallet could exist,
//! which blocked setup for anyone who only wanted to receive or deposit. Now a
//! wallet is a directory and a keystore, and it opens, gives its address, locks
//! and unlocks with nothing else set.

use nox_shield_core::custody::HardwareGuard;
use nox_shield_core::error::CustodyError;
use nox_shield_core::ffi::Wallet;
use std::sync::Arc;

struct Paper;

impl HardwareGuard for Paper {
    fn wrap(&self, key: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        let mut out = vec![0xA5];
        out.extend_from_slice(&key);
        Ok(out)
    }

    fn unwrap_key(&self, blob: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        match blob.split_first() {
            Some((0xA5, rest)) => Ok(rest.to_vec()),
            _ => Err(CustodyError::Guard),
        }
    }
}

#[test]
fn a_wallet_with_nothing_configured_opens_receives_and_unlocks() {
    let dir = std::env::temp_dir().join(format!("nox-norelay-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let wallet = Wallet::new(dir.to_string_lossy().into_owned(), Arc::new(Paper))
        .expect("a wallet under a temporary directory");
    wallet.create().expect("a wallet");
    let address = wallet.receiving_address().expect("an address to receive at");
    assert!(address.starts_with("nox1"), "{address}");
    wallet.lock().expect("lock");
    wallet.unlock().expect("unlock with nothing configured");
    assert!(wallet.wipe().expect("wipe"));
    std::fs::remove_dir_all(&dir).ok();
}
