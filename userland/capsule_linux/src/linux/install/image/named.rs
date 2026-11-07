//! The listings installed from an image: `linux.nonos-image-<package>`,
//! which the kernel passes as `nonos-image-<package>`, inside the in-tree
//! tools' namespace (tools.rs), so the catalogue's reserved names cover it.
//! The mirror serves an image and its chunk list at paths named by the pin.
//! Pure, so the host proofs hold the names and paths.

use alloc::string::String;

pub const PREFIX: &str = "nonos-image-";

/// The package `pkg`'s image sells, or None when `pkg` names no image.
pub fn image(pkg: &str) -> Option<&str> {
    let name = pkg.strip_prefix(PREFIX)?;
    let fits = |c: u8| c.is_ascii_lowercase() || c.is_ascii_digit() || b"._+-".contains(&c);
    let first = name.bytes().next().is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    (first && name.len() <= 64 && name.bytes().all(fits)).then_some(name)
}

/// The image package a bare word names, `nonos-image-<word>`: None for a
/// word with a path in it, one already an image's, or one no image could be.
pub fn of_word(word: &[u8]) -> Option<String> {
    let word = core::str::from_utf8(word).ok().filter(|w| !w.starts_with(PREFIX))?;
    let pkg = [PREFIX, word].concat();
    image(&pkg).is_some().then_some(pkg)
}

/// Where a mirror serves the chunk list pinned as `pin`.
pub fn list_path(pin: &[u8; 32]) -> String {
    path(pin, ".chunks")
}

/// Where a mirror serves the image the list pinned as `pin` describes.
pub fn image_path(pin: &[u8; 32]) -> String {
    path(pin, ".image")
}

/// The pin as the 64 lower-case hex digits paths and the cache name it by.
pub fn hex(pin: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in pin {
        s.push(char::from(b"0123456789abcdef"[usize::from(b >> 4)]));
        s.push(char::from(b"0123456789abcdef"[usize::from(b & 15)]));
    }
    s
}

fn path(pin: &[u8; 32], ext: &str) -> String {
    alloc::format!("/linux/{}{ext}", hex(pin))
}
