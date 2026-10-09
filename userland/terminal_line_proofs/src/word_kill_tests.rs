// NONOS Operating System (AGPL-3.0-or-later)
/*
 * Alt-D, the forward twin of Ctrl-W: it cuts what Alt-F would step over,
 * and what it cut comes back with Ctrl-Y.
 */

use crate::line::Line;

fn typed(s: &str) -> Line {
    let mut l = Line::new();
    for ch in s.chars() {
        l.insert_char(ch);
    }
    l
}

fn text(l: &Line) -> String {
    String::from_utf8(l.as_bytes().to_vec()).unwrap()
}

#[test]
fn alt_d_cuts_the_word_and_its_gap() {
    let mut l = typed("git push origin main");
    l.move_home();
    l.move_word_right();
    assert!(l.delete_word_right());
    assert_eq!(text(&l), "git origin main");
    assert_eq!(l.cursor, 4);
}

#[test]
fn alt_d_agrees_with_alt_f() {
    let mut a = typed("ls -la /data");
    a.move_home();
    let mut b = typed("ls -la /data");
    b.move_home();
    b.move_word_right();
    assert!(a.delete_word_right());
    assert_eq!(text(&a).len(), "ls -la /data".len() - b.cursor);
}

#[test]
fn alt_d_at_the_end_does_nothing() {
    let mut l = typed("echo hi");
    assert!(!l.delete_word_right());
    assert_eq!(text(&l), "echo hi");
}

#[test]
fn what_alt_d_cut_yanks_back() {
    let mut l = typed("rm -rf /tmp/x");
    l.move_home();
    assert!(l.delete_word_right());
    assert_eq!(text(&l), "-rf /tmp/x");
    assert!(l.yank());
    assert_eq!(text(&l), "rm -rf /tmp/x");
}

#[test]
fn alt_d_mid_word_cuts_its_tail() {
    let mut l = typed("cargo build");
    l.move_home();
    l.move_right();
    l.move_right();
    assert!(l.delete_word_right());
    assert_eq!(text(&l), "cabuild");
}
