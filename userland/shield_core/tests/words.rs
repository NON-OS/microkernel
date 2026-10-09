/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used)]

//! The recovery words come back from the vault after the keystore confirms the owner.

mod keystore;
mod restored;

use restored::restored;

#[test]
fn the_words_come_back_after_a_restore() {
    let dir = std::env::temp_dir().join(format!("nox-words-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    let wallet = restored(&dir);
    let words = wallet.recovery_words().expect("the words").expect("a version 2 vault");
    assert_eq!(words.len(), 12);
    assert_eq!(words.last().map(String::as_str), Some("about"));
    assert!(words.iter().take(11).all(|w| w == "abandon"));
    wallet.wipe().expect("the wipe");
    std::fs::remove_dir_all(&dir).ok();
}
