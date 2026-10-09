/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

//! Accounts and key export through the wallet the shells hold, on the public 12-word phrase, end
//! to end: the addresses are the standard ones at index 0 and 1, and each exported key controls its own.

mod keystore;
mod restored;

use nox_shield_core::error::WalletError;
use restored::restored;

/// The address a private key in `0x` hex controls, through k256 and Keccak.
fn address_of(key: &str) -> String {
    use sha3::{Digest, Keccak256};
    let bytes: Vec<u8> =
        (2..66).step_by(2).map(|i| u8::from_str_radix(&key[i..i + 2], 16).expect("hex")).collect();
    let secret = k256::ecdsa::SigningKey::from_slice(&bytes).expect("a key on the curve");
    let point = secret.verifying_key().to_encoded_point(false);
    let digest = Keccak256::digest(&point.as_bytes()[1..]);
    nox_shield_core::evm::checksummed(&digest[12..].try_into().expect("20 bytes"))
}

#[test]
fn accounts_are_added_chosen_kept_and_exported() {
    let dir = std::env::temp_dir().join(format!("nox-accounts-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let wallet = restored(&dir);
    assert_eq!(wallet.accounts().expect("accounts").len(), 1);
    let first = wallet.public_address().expect("the first address");
    assert_eq!(first, "0x9858EfFD232B4033E47d90003D41EC34EcaEda94");
    assert_eq!(wallet.add_account().expect("a second account"), 1);
    let second = wallet.public_address().expect("the second, now active");
    assert_eq!(second, "0x6Fac4D18c912343BF86fa7049364Dd4E424Ab9C0");
    assert_ne!(first, second);
    wallet.select_account(0).expect("back to the first");
    assert_eq!(wallet.public_address().expect("the active address"), first);
    assert_eq!(wallet.select_account(2), Err(WalletError::NoSuchAccount));
    wallet.lock().expect("the lock");
    wallet.unlock().expect("the unlock");
    let listed = wallet.accounts().expect("accounts");
    assert_eq!(listed.len(), 2, "the second account outlives a lock");
    assert!(listed[0].active && listed[1].public_address == second);
    for summary in &listed {
        let key = wallet.export_public_key(summary.index).expect("the key");
        assert_eq!(address_of(&key), summary.public_address, "the key controls its account");
    }
    assert_eq!(wallet.export_public_key(2), Err(WalletError::NoSuchAccount));
    let first_nox1 = wallet.receiving_address().expect("nox1");
    wallet.select_account(1).expect("the second");
    assert_ne!(wallet.receiving_address().expect("nox1"), first_nox1, "each account has its own");
    wallet.wipe().expect("the wipe");
    std::fs::remove_dir_all(&dir).ok();
}
