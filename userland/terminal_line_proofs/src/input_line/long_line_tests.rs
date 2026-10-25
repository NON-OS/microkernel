// NONOS Operating System (AGPL-3.0-or-later)
//! How long a command the line editor takes.

use crate::line::Line;

fn typed(s: &str) -> Line {
    let mut l = Line::new();
    for ch in s.chars() {
        l.insert_char(ch);
    }
    l
}

/*
 * A command wider than the screen is typed whole. The buffer used to be the
 * screen's width, so an echo with a redirect lost its file name at column
 * 96 and wrote somewhere else.
 */
#[test]
fn a_command_wider_than_the_screen_is_kept_whole() {
    let cmd = format!("echo '{}' > /tmp/boot.json", "x".repeat(200));
    let l = typed(&cmd);
    assert_eq!(l.as_bytes(), cmd.as_bytes());
    assert!(cmd.len() > crate::term::dimensions::COLS);
}
