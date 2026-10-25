// NONOS Operating System (AGPL-3.0-or-later)
//! What the prompt shows of a search: the mode, the needle, and no match.

use super::search_head::{head, width, FOUND, GAP, MISSING};

#[test]
fn the_needle_is_shown_whole_whatever_the_directory() {
    // At `~` the old prompt cut the needle to one cell: `he` showed as `h`.
    let (label, shown) = head(b"he", true, 40);
    assert_eq!((label, shown), (FOUND, &b"he"[..]));
    assert_eq!(width(label, shown), FOUND.len() + 2 + GAP);
}

#[test]
fn no_match_is_said() {
    assert_eq!(head(b"zz", false, 40).0, MISSING);
}

#[test]
fn a_long_needle_keeps_its_end_and_whole_characters() {
    let room = FOUND.len() + GAP + 4;
    assert_eq!(head(b"abcdefgh", true, room).1, b"efgh");
    let (_, shown) = head("aé€bc".as_bytes(), true, room);
    assert_eq!(core::str::from_utf8(shown).unwrap(), "é€bc");
    let (_, shown) = head("aé€bcd".as_bytes(), true, room);
    assert_eq!(core::str::from_utf8(shown).unwrap(), "€bcd");
    assert_eq!(head(b"abc", true, 1).1, b"", "no room shows no needle");
}
