// NONOS Operating System (AGPL-3.0-or-later)
//! The console line: pieces held until the newline, then one write; the
//! shared files' tag replaced by this driver's; a long line cut, still one
//! write ending in a newline; numbers built without allocating.

use crate::status::digits::{decimal, hex16};
use crate::status::line::{Line, LINE_MAX};

fn feed(pieces: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut l = Line::resume([0; LINE_MAX], 0);
    let mut out = Vec::new();
    for p in pieces {
        l.push(p, |w| out.push(w.to_vec()));
    }
    out
}

#[test]
fn pieces_are_one_write() {
    let out = feed(&[b"[rtl8822ce] firmware v", b"9", b".", b"9", b"\n"]);
    assert_eq!(out, vec![b"[rtl8822ce] firmware v9.9\n".to_vec()]);
}

#[test]
fn nothing_is_written_before_the_newline() {
    assert!(feed(&[b"[rtl8822ce] half a line"]).is_empty());
}

#[test]
fn the_shared_tag_is_this_drivers() {
    let out = feed(&[b"[rtl8821ce] serving net_core\n"]);
    assert_eq!(out, vec![b"[rtl8822ce] serving net_core\n".to_vec()]);
}

#[test]
fn only_a_leading_tag_is_replaced() {
    let out = feed(&[b"x [rtl8821ce] y\n", b"[rtl8821c] z\n"]);
    assert_eq!(out, vec![b"x [rtl8821ce] y\n".to_vec(), b"[rtl8821c] z\n".to_vec()]);
}

#[test]
fn a_long_line_is_cut_and_ends_its_line() {
    let long = [b'a'; 400];
    let out = feed(&[&long, b"\n", b"next\n"]);
    assert_eq!(out.len(), 2);
    assert_eq!(out[0].len(), LINE_MAX);
    assert_eq!(*out[0].last().unwrap(), b'\n');
    assert_eq!(out[1], b"next\n".to_vec());
}

#[test]
fn two_lines_in_one_piece_are_two_writes() {
    let out = feed(&[b"a\nb\n"]);
    assert_eq!(out, vec![b"a\n".to_vec(), b"b\n".to_vec()]);
}

#[test]
fn a_held_line_resumes() {
    let mut l = Line::resume([0; LINE_MAX], 0);
    l.push(b"[rtl8821ce] cr=", |_| panic!("written early"));
    let (buf, len) = l.held();
    let mut out = Vec::new();
    Line::resume(*buf, len).push(b"00ea\n", |w| out.push(w.to_vec()));
    assert_eq!(out, vec![b"[rtl8822ce] cr=00ea\n".to_vec()]);
}

#[test]
fn numbers_read_as_written() {
    let mut d = [0; 10];
    assert_eq!(decimal(0, &mut d), b"0");
    assert_eq!(decimal(4_294_967_295, &mut d), b"4294967295");
    assert_eq!(&hex16(0x00EA), b"00ea");
}
