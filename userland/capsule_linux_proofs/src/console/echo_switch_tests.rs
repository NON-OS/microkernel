// NONOS Operating System (AGPL-3.0-or-later)
//! A password prompt turns ECHO off and keeps canonical mode: the terminal
//! hears ?7728 exactly when echo goes off or comes back, so the typed
//! password is not shown, and nothing when only another flag changes.

use super::tty_rules::{echo_switch, mode_switch};

fn termios(lflag_low: u8) -> [u8; 36] {
    let mut t = [0u8; 36];
    t[12] = lflag_low;
    t
}

const COOKED: u8 = 0o073; // ISIG ICANON ECHO ECHOE ECHOK
const PASSWORD: u8 = 0o063; // golang.org/x/term ReadPassword: ECHO off, ICANON kept

#[test]
fn a_password_prompt_hides_what_is_typed_and_shows_it_again_after() {
    let (cooked, password) = (termios(COOKED), termios(PASSWORD));
    assert_eq!(echo_switch(&cooked, &password), Some(&b"\x1b[?7728h"[..]));
    assert_eq!(echo_switch(&password, &cooked), Some(&b"\x1b[?7728l"[..]));
    assert_eq!(mode_switch(&cooked, &password), None, "still canonical: lines, not raw keys");
}

#[test]
fn echo_is_told_only_when_it_changes() {
    let (cooked, password) = (termios(COOKED), termios(PASSWORD));
    assert_eq!(echo_switch(&cooked, &cooked), None);
    assert_eq!(echo_switch(&password, &password), None);
    assert_eq!(echo_switch(&cooked, &termios(COOKED & !0o001)), None, "ISIG alone");
}
