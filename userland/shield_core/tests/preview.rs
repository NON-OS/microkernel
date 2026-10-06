//! The `nox1` address a wallet shows before its store is made is the one the store answers:
//! for account 0 of the standard test phrase, for a further account, and for a wallet from
//! the phrase's index 0 key. The preview opens no store and reads nothing from the network.

mod keystore;
mod restored;

use keystore::{scratch, TestGuard};
use nox_shield_core::ffi::{address_of_key, address_of_words, Wallet};
use restored::restored;
use std::sync::Arc;

const KEY: &str = "0x1ab42cc412b618bdea3a599e3c9bae199ebf030895b039e9db1e30dafb12b727";
const ADDRESS: &str = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";

fn words() -> Vec<String> {
    let mut words = vec!["abandon".to_string(); 11];
    words.push("about".to_string());
    words
}

fn public(hex: &str) -> [u8; 20] {
    let hex = hex.trim_start_matches("0x");
    let mut out = [0u8; 20];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("hex");
    }
    out
}

#[test]
fn the_preview_of_account_zero_is_the_restored_stores_address() {
    let wallet = restored(&scratch("preview-zero"));
    let stored = wallet.receiving_address().expect("nox1");
    let preview = address_of_words(&words(), public(ADDRESS)).expect("the words");
    assert_eq!(preview.as_deref(), Some(stored.as_str()));
    assert!(stored.starts_with("nox1"));
}

#[test]
fn the_preview_of_a_further_account_is_that_accounts_address() {
    let wallet = restored(&scratch("preview-further"));
    wallet.add_account().expect("account 1");
    let accounts = wallet.accounts().expect("accounts");
    let second = accounts.iter().find(|a| a.index == 1).expect("account 1");
    wallet.select_account(1).expect("select");
    let stored = wallet.receiving_address().expect("nox1");
    let preview = address_of_words(&words(), public(&second.public_address)).expect("the words");
    assert_eq!(preview.as_deref(), Some(stored.as_str()));
}

#[test]
fn the_preview_of_a_key_wallet_is_its_stores_address() {
    let dir = scratch("preview-key");
    let wallet = Wallet::new(dir.to_string_lossy().into_owned(), Arc::new(TestGuard)).expect("w");
    wallet.restore_from_key(KEY.into()).expect("restored");
    let stored = wallet.receiving_address().expect("nox1");
    let preview = address_of_key(KEY, public(ADDRESS)).expect("the key");
    assert_eq!(preview.as_deref(), Some(stored.as_str()));
}

#[test]
fn words_or_a_key_for_another_account_give_no_preview() {
    let other = [0x11u8; 20];
    assert_eq!(address_of_words(&words(), other).expect("the words"), None);
    assert_eq!(address_of_key(KEY, other).expect("the key"), None);
}
