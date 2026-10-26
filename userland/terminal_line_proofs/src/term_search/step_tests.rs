// NONOS Operating System (AGPL-3.0-or-later)
//! Ctrl+R as a person drives it: the keys go to the needle, Esc leaves with
//! the line put back, and Enter runs only a line the needle still matches.

use super::history::History;
use super::line::Line;
use super::search_step::{begin, escape, older, retype, runs};

fn ran(lines: &[&str]) -> History {
    let mut h = History::new();
    lines.iter().for_each(|l| h.push(l.as_bytes()));
    h
}

fn typed(s: &str) -> Line {
    let mut l = Line::new();
    s.chars().for_each(|c| assert!(l.insert_char(c)));
    l
}

fn shows(l: &Line) -> &str {
    core::str::from_utf8(l.as_bytes()).unwrap()
}

#[test]
fn a_needle_finds_the_newest_line_that_holds_it() {
    let h = ran(&["help", "ls -l /", "cat help.txt", "pwd"]);
    let mut l = typed("draft");
    let mut s = begin(&mut l, &h);
    assert_eq!(shows(&l), "pwd", "an empty needle shows the newest line");
    s.needle.extend_from_slice(b"he");
    retype(&mut s, &h, &mut l);
    assert_eq!(shows(&l), "cat help.txt");
    assert!(runs(&s));
    older(&mut s, &h, &mut l);
    assert_eq!(shows(&l), "help");
    older(&mut s, &h, &mut l);
    assert_eq!(shows(&l), "help", "with nothing older the match stays");
    assert!(runs(&s));
}

#[test]
fn a_needle_with_no_match_runs_nothing() {
    let h = ran(&["help", "pwd"]);
    let mut l = typed("");
    let mut s = begin(&mut l, &h);
    s.needle.extend_from_slice(b"zz");
    retype(&mut s, &h, &mut l);
    assert!(!runs(&s), "Enter must not run the older match on the line");
    s.needle.truncate(0);
    s.needle.extend_from_slice(b"pw");
    retype(&mut s, &h, &mut l);
    assert!(runs(&s), "fixing the needle finds again");
}

#[test]
fn esc_leaves_the_search_and_puts_the_line_back() {
    let h = ran(&["help"]);
    let mut l = typed("ls -l /");
    let mut s = Some(begin(&mut l, &h));
    assert_eq!(shows(&l), "help");
    escape(&mut s, &mut l);
    assert!(s.is_none(), "the next keys go to the line, not the needle");
    assert_eq!(shows(&l), "ls -l /");
    assert_eq!(l.cursor, l.len);
    escape(&mut s, &mut l);
    assert_eq!(shows(&l), "", "with no search Esc clears the line");
}
