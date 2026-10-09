//! A wallet restored from the public 12-word phrase "abandon" eleven times then "about", the
//! phrase every wallet tests with, in a directory of the test's own.

use super::keystore::TestGuard;
use nox_shield_core::ffi::Wallet;
use std::sync::Arc;

pub fn restored(dir: &std::path::Path) -> Arc<Wallet> {
    let wallet = Wallet::new(dir.to_string_lossy().into_owned(), Arc::new(TestGuard))
        .expect("a wallet under a temporary directory");
    let mut words = vec!["abandon".to_string(); 11];
    words.push("about".to_string());
    wallet.restore(words).expect("the standard phrase");
    wallet
}
