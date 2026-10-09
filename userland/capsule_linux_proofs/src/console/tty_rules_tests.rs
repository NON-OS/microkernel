// NONOS Operating System (AGPL-3.0-or-later)
//! The console's tty rules: the terminal hears ?7727 exactly when a guest
//! leaves or re-enters canonical mode, and Ctrl+C is a signal only while
//! ISIG is set, as BusyBox's line editor clears it at the prompt.

use super::queue::Queue;
use super::tty_rules::{holds, interrupt_byte, interrupt_line, mode_switch};

/// Linux's cooked defaults for c_lflag's low byte and VINTR.
fn termios(lflag_low: u8, vintr: u8) -> [u8; 36] {
    let mut t = [0u8; 36];
    t[12] = lflag_low;
    t[17] = vintr;
    t
}

const COOKED: u8 = 0o073; // ISIG ICANON ECHO ECHOE ECHOK
const ASH_RAW: u8 = 0o060; // lineedit: ICANON, ECHO and ISIG off

#[test]
fn the_terminal_is_told_only_on_a_change_of_canonical_mode() {
    let cooked = termios(COOKED, 3);
    let raw = termios(ASH_RAW, 3);
    assert_eq!(mode_switch(&cooked, &raw), Some(&b"\x1b[?7727h"[..]));
    assert_eq!(mode_switch(&raw, &cooked), Some(&b"\x1b[?7727l"[..]));
    assert_eq!(mode_switch(&cooked, &cooked), None);
    assert_eq!(mode_switch(&raw, &raw), None);
}

#[test]
fn ctrl_c_is_a_signal_only_while_isig_is_set() {
    assert_eq!(interrupt_byte(&termios(COOKED, 3), true), Some(3));
    assert_eq!(interrupt_byte(&termios(ASH_RAW, 3), true), None, "the prompt reads ^C itself");
    assert_eq!(interrupt_byte(&termios(COOKED, 0), true), None, "VINTR disabled");
}

#[test]
fn input_from_a_file_is_never_an_interrupt() {
    // `linux zstd -d < f.zst`: stdin is the file, and its 0x03 bytes are data.
    assert_eq!(interrupt_byte(&termios(COOKED, 3), false), None);
}

#[test]
fn an_interrupt_is_found_anywhere_unread() {
    let mut q = Queue::new();
    q.push(b"sleep 9");
    assert!(!holds(&q, 3));
    q.push(&[3]);
    assert!(holds(&q, 3));
}

#[test]
fn each_ctrl_c_says_its_group_and_how_many_took_it() {
    assert_eq!(interrupt_line(7, 1), "[LINUX] Ctrl+C: SIGINT to group 7, 1 processes\n");
    assert_eq!(interrupt_line(3, 0), "[LINUX] Ctrl+C: SIGINT to group 3, 0 processes\n");
}
