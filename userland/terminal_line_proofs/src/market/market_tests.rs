// NONOS Operating System (AGPL-3.0-or-later)
//! `linux/htop  htop  [installed]`: the repository split off, names lined up,
//! the standing's colour matching its word, and no colour in a file.

use super::list_row::{heading, row, tone};

fn text(b: &[u8]) -> &str {
    std::str::from_utf8(b).unwrap()
}

#[test]
fn the_heading_counts_in_words() {
    assert_eq!(text(&heading(12).0), ":: 12 listings");
    assert_eq!(text(&heading(1).0), ":: 1 listing");
    assert!(heading(3).1.starts_with(b"\x1b[1;34m::"), "the :: in blue, as pacman");
}

#[test]
fn the_repository_is_split_off_and_names_line_up() {
    let (a, _) = row(b"linux.htop", b"htop", "installed", 16);
    let (b, _) = row(b"linux.qwen-small", b"Qwen small", "ready", 16);
    assert_eq!(text(&a), "linux/htop        htop  [installed]");
    assert_eq!(text(&a).find("htop  ["), Some(18), "names start in one column");
    assert_eq!(text(&b).find("Qwen"), Some(18));
    assert_eq!(text(&row(b"plain", b"P", "ready", 5).0), "plain  P  [ready]", "no dot, no repo");
}

#[test]
fn each_standing_has_its_own_colour() {
    let tones = ["installed", "ready", "installing", "failed", "not ready"].map(tone);
    for (i, a) in tones.iter().enumerate() {
        for b in &tones[i + 1..] {
            assert_ne!(a, b);
        }
    }
    assert_eq!(tone("removing"), tone("installing"), "both are moving");
    assert_eq!(tone("installed"), b"\x1b[1;32m");
    assert_eq!(tone("failed"), b"\x1b[1;31m");
}

#[test]
fn a_file_gets_the_words_and_no_colour() {
    let (plain, styled) = row(b"linux.htop", b"htop", "failed", 10);
    assert!(!plain.contains(&0x1b) && !heading(2).0.contains(&0x1b));
    assert!(styled.windows(5).any(|w| w == b"[1;31"), "the tag in red on screen");
    assert!(styled.ends_with(b"\x1b[0m"));
}
