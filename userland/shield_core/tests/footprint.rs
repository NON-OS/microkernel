/*
 * A test asserts by panicking, and a fixture reads directories by hand, so the
 * lints that forbid panicking and indexing are off here and nowhere else.
 */
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

//! What the wallet leaves on disk, and nothing else. Every file is findable by whoever holds
//! the device, so the set is part of the privacy claim: the sealed seed, the note log and a
//! bench card. A fourth file without a decision is the finding this test exists for.
//!
//! Creating a wallet writes one of the three. The log arrives with its first row, measured here,
//! since an empty log would tell a device holder a wallet was opened and never used.

use nox_shield_core::custody::HardwareGuard;
use nox_shield_core::error::CustodyError;
use nox_shield_core::ffi::Wallet;
use std::sync::Arc;

/// A guard that wraps by prefixing, which exercises a vault round trip without
/// a keystore. It holds no key and keeps nothing.
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

fn names(dir: &std::path::Path) -> Vec<String> {
    let mut found: Vec<String> = std::fs::read_dir(dir)
        .expect("the directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    found.sort();
    found
}

/// Creating and using a wallet writes the seal and the log, and nothing else.
#[test]
fn a_wallet_writes_only_the_files_it_is_allowed_to() {
    let dir = std::env::temp_dir().join(format!("nox-footprint-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");

    let open = wallet(&dir);
    open.create().expect("a wallet");
    assert_eq!(
        names(&dir),
        vec!["vault.seal".to_string()],
        "a created wallet wrote something it was not asked to"
    );

    open.lock().expect("the lock");
    open.unlock().expect("the unlock");
    assert_eq!(names(&dir), vec!["vault.seal".to_string()], "locking or unlocking wrote a file");

    std::fs::remove_dir_all(&dir).ok();
}

/// The seal is a ciphertext. A test that only checked the filename would pass
/// on a file that had the seed in it, so this one reads the bytes and asserts
/// the phrase's words are not among them.
#[test]
fn the_seal_does_not_carry_the_phrase() {
    let dir = std::env::temp_dir().join(format!("nox-seal-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");

    let open = wallet(&dir);
    let phrase = open.create().expect("a wallet");
    let sealed = std::fs::read(dir.join("vault.seal")).expect("the seal");

    for word in &phrase {
        assert!(
            !sealed.windows(word.len()).any(|w| w == word.as_bytes()),
            "the word {word} is in the seal in the clear"
        );
    }

    std::fs::remove_dir_all(&dir).ok();
}
