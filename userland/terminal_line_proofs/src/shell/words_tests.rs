// NONOS Operating System (AGPL-3.0-or-later)
//! A line keeps every word it holds: `echo` and eight words printed seven,
//! the ninth dropped with nothing said. Up to 64 are kept; past that the line
//! is marked cut, so it is refused rather than run short.

use crate::parse::{parse, TOO_MANY};

#[test]
fn the_fox_keeps_its_lazy_dog() {
    let a = parse(b"echo the quick brown fox jumps over the lazy dog");
    assert_eq!(a.argc, 10);
    assert_eq!(a.argv[9], b"dog");
    assert!(!a.cut);
}

#[test]
fn sixty_four_words_are_kept_and_the_sixty_fifth_cuts() {
    let words = |n: usize| (0..n).map(|i| format!("w{i}")).collect::<Vec<_>>().join(" ");
    let full = words(64);
    let a = parse(full.as_bytes());
    assert_eq!((a.argc, a.cut), (64, false));
    let over = words(65);
    let b = parse(over.as_bytes());
    assert!(b.cut, "the 65th word is not dropped quietly");
    assert!(!parse(format!("{full}   ").as_bytes()).cut, "trailing spaces are no word");
    assert!(String::from_utf8_lossy(TOO_MANY).contains("nothing ran"));
}
