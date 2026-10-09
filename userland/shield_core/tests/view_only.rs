/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used, clippy::indexing_slicing)]

//! A view key given to the wallet becomes a view-only account: kept sealed across a lock, one
//! account however often it is pasted, and gone with the wipe.

mod keystore;
mod restored;

use nox_shield_core::error::WalletError;
use nox_shield_core::keys::ViewKind;
use restored::restored;

/// The full view key of account 0 of the test phrase, the vector in docs/09-accounts.md.
const FULL_KEY: &str = concat!(
    "noxfvk1aedb3jxfu4aojpai7gh2uoy4qt5lrko2skq4cl5xopdlzhzws6xkit4vmdzg4ffns3phya3dknt6b",
    "xklxypbmf6ia32vbgtkbmqbrtlujcshnfqde7moxavqzyluitm24v2mqfguzqoqr5bwsskf3dwaepm2bgqieuaaaaaa",
);

#[test]
fn a_view_key_becomes_a_view_only_account_once() {
    let dir = std::env::temp_dir().join(format!("nox-watch-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let wallet = restored(&dir);
    let key = FULL_KEY.to_string();
    let exported = wallet.export_view_key(0, ViewKind::Full);
    if cfg!(feature = "view-key-export") {
        assert_eq!(exported.expect("a full view key"), key, "the export is the vector");
    } else {
        assert_eq!(exported, Err(WalletError::Unavailable), "no export in this build");
    }
    assert_eq!(wallet.import_view_key(key.clone()).expect("imported"), 0);
    assert_eq!(wallet.import_view_key(key).expect("the same key"), 0, "one key, one account");
    assert!(wallet.import_view_key("noxivk1aaaa".into()).is_err());
    wallet.lock().expect("the lock");
    wallet.unlock().expect("the unlock");
    let watched = wallet.watched_accounts().expect("view-only accounts");
    assert_eq!(watched.len(), 1, "the view key outlives a lock, sealed");
    assert_eq!(watched[0].kind, ViewKind::Full);
    let sealed = std::fs::read(dir.join("watching.seal")).expect("the sealed file");
    assert!(
        !sealed.windows(7).any(|w| w == b"noxfvk1"),
        "the view key is not on disk in the clear"
    );
    wallet.forget_view_key(0).expect("forgotten");
    assert!(wallet.watched_accounts().expect("none").is_empty());
    wallet.wipe().expect("the wipe");
    let left: Vec<_> = std::fs::read_dir(&dir).expect("the directory").flatten().collect();
    assert!(left.is_empty(), "the wipe left a file behind");
    std::fs::remove_dir_all(&dir).ok();
}
