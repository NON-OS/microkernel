//! The rehearsal wallet, kept in one directory across stages, from a phrase with a public label.

pub mod stage;
mod wait;

use super::keystore::TestGuard;
use nox_shield_core::ffi::Wallet;
use std::sync::Arc;

/// The label the rehearsal phrase is drawn from. Anyone can rebuild it, so it guards test money.
const LABEL: &str = "nonos wallet end-to-end rehearsal 1";

pub fn words() -> Vec<String> {
    let entropy = *blake3::hash(LABEL.as_bytes()).as_bytes();
    let mut indices = [0u16; 24];
    let n = nonos_hd::bip39::entropy_to_words(&entropy, &mut indices).expect("24 words");
    indices[..n].iter().map(|i| nonos_hd::ENGLISH_WORDLIST[usize::from(*i)].to_string()).collect()
}

/// The wallet, restored the first time and unlocked after.
pub fn wallet() -> Arc<Wallet> {
    let dir = std::env::var("E2E_DIR").expect("E2E_DIR, kept between stages");
    std::fs::create_dir_all(&dir).expect("the directory");
    let wallet = Wallet::new(dir, Arc::new(TestGuard)).expect("a wallet");
    if wallet.stored() {
        wallet.unlock().expect("unlocked");
    } else {
        wallet.restore(words()).expect("restored");
        wallet.add_account().expect("account 2");
        wallet.add_account().expect("account 3, the withdrawal target");
        wallet.select_account(0).expect("account 1");
    }
    wallet
}
