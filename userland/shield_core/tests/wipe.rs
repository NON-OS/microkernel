/*
 * A test asserts by panicking, and a fixture writes files by hand, so the
 * lints that forbid panicking and indexing are off here and nowhere else.
 */
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

//! Destroying a wallet leaves nothing behind that a later open could read.
//!
//! This path loses money, so it must never report success while a file survives, and never
//! fail on a device that had no wallet.

use nox_shield_core::custody::HardwareGuard;
use nox_shield_core::error::CustodyError;
use nox_shield_core::ffi::Wallet;
use std::sync::Arc;

/// A guard that wraps by prefixing, which is enough to exercise a vault round
/// trip without a keystore. It holds no key and keeps nothing.
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

fn wallet(dir: &std::path::Path) -> Arc<Wallet> {
    Wallet::new(dir.to_string_lossy().into_owned(), Arc::new(Paper))
        .expect("a wallet under a temporary directory")
}

#[test]
fn a_wiped_wallet_leaves_nothing_to_open() {
    let dir = std::env::temp_dir().join(format!("nox-wipe-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");

    let open = wallet(&dir);
    open.create().expect("a wallet");
    assert!(open.stored(), "a created wallet is stored");

    assert!(open.wipe().expect("the wipe"), "the wipe reports it destroyed one");
    assert!(!open.stored(), "nothing is stored after a wipe");

    let files: Vec<_> = std::fs::read_dir(&dir)
        .expect("the directory")
        .filter_map(|entry| entry.ok().map(|e| e.file_name()))
        .collect();
    assert!(files.is_empty(), "the wipe left files behind: {files:?}");

    let again = wallet(&dir);
    assert!(again.unlock().is_err(), "a wiped vault cannot be unlocked");

    std::fs::remove_dir_all(&dir).ok();
}

/// A device that never had a wallet is not an error. The caller asked for
/// there to be none.
#[test]
fn wiping_nothing_is_not_a_failure() {
    let dir = std::env::temp_dir().join(format!("nox-wipe-empty-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let open = wallet(&dir);
    assert!(!open.wipe().expect("the wipe"), "there was nothing to destroy");
    std::fs::remove_dir_all(&dir).ok();
}
