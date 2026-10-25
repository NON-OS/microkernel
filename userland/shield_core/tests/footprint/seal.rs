//! The sealed seed holds none of the phrase's words in the clear.

use super::wallet;

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
