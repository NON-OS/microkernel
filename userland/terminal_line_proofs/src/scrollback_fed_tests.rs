// NONOS Operating System (AGPL-3.0-or-later)
//! The scrollback counts each write that reaches the screen, and only those:
//! a job's tick repaints the window only when the count moved.

use crate::scrollback::Scrollback;

#[test]
fn a_write_moves_the_count() {
    let mut sb = Scrollback::new();
    let at = sb.fed;
    sb.feed_raw(b"hello\n");
    assert_ne!(sb.fed, at);
}

#[test]
fn nothing_written_leaves_it() {
    let mut sb = Scrollback::new();
    let at = sb.fed;
    sb.feed_raw(b"");
    assert_eq!(sb.fed, at);
}

#[test]
fn every_way_onto_the_screen_moves_it() {
    let mut sb = Scrollback::new();
    let mut at = sb.fed;
    let mut moved = |sb: &Scrollback| {
        let m = sb.fed != at;
        at = sb.fed;
        m
    };
    sb.push_line(b"a line");
    assert!(moved(&sb));
    sb.push_error(b"an error");
    assert!(moved(&sb));
    sb.push_styled(b"plain", b"\x1b[32mstyled");
    assert!(moved(&sb));
    sb.push_dir_row(b"drwx ", b"dir");
    assert!(moved(&sb));
    sb.program_ended();
    assert!(moved(&sb));
    sb.clear();
    assert!(moved(&sb));
}

#[test]
fn a_capture_is_not_the_screen() {
    let mut sb = Scrollback::new();
    sb.begin_capture();
    let at = sb.fed;
    sb.push_line(b"to a file");
    assert_eq!(sb.fed, at);
    assert_eq!(sb.end_capture(), vec![b"to a file".to_vec()]);
}
