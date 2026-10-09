// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The process monitor never leaves its table card blank and never swallows
//! the answer to a kill: the card names why it has no rows, and the status
//! strip carries the last action's prompt or outcome, or an unreadable table,
//! ahead of the key hint.

use crate::pm_critical::protected;
use crate::pm_format::percent;
use crate::pm_notes::{
    empty_table, kill_note, strip_note, ARMED, KEYS_HINT, LIVE, NO_SELECTION, PROTECTED, READING,
    UNAVAILABLE,
};

#[test]
fn a_table_with_rows_to_draw_has_no_note() {
    assert_eq!(empty_table(12, 12, LIVE), None);
    assert_eq!(empty_table(12, 1, LIVE), None);
}

#[test]
fn a_table_filtered_to_nothing_says_so_rather_than_looking_empty() {
    let note = empty_table(12, 0, LIVE).expect("a note");
    assert!(note.starts_with(b"No process matches"));
}

#[test]
fn a_table_still_being_read_says_so() {
    assert_eq!(empty_table(0, 0, READING), Some(READING));
}

#[test]
fn a_table_the_kernel_refused_names_the_refusal() {
    assert_eq!(empty_table(0, 0, UNAVAILABLE), Some(UNAVAILABLE));
    assert!(UNAVAILABLE.starts_with(b"process table unavailable"));
}

#[test]
fn a_live_read_with_no_rows_is_not_reported_as_live() {
    let note = empty_table(0, 0, LIVE).expect("a note");
    assert_ne!(note, LIVE);
    assert_eq!(note, b"The kernel listed no processes");
}

#[test]
fn a_kill_prompt_or_outcome_reaches_the_status_strip() {
    for notice in [ARMED, kill_note(-1), kill_note(0), PROTECTED, NO_SELECTION] {
        assert_eq!(strip_note(notice, LIVE), notice);
        assert_eq!(strip_note(notice, UNAVAILABLE), notice);
    }
}

#[test]
fn an_unreadable_table_reaches_the_status_strip_when_nothing_else_is_pending() {
    assert_eq!(strip_note(b"", UNAVAILABLE), UNAVAILABLE);
}

#[test]
fn the_key_hint_shows_only_when_there_is_nothing_to_report() {
    assert_eq!(strip_note(b"", LIVE), KEYS_HINT);
    assert_eq!(strip_note(b"", READING), KEYS_HINT);
}

#[test]
fn an_end_the_kernel_carried_out_says_ended() {
    assert_eq!(kill_note(0), b"ended");
}

#[test]
fn an_end_the_kernel_refused_says_why_not_that_it_happened() {
    assert_eq!(kill_note(-1), b"denied by the kernel: no authority over that process");
    assert_eq!(kill_note(-22), b"the kernel refused the request as invalid");
    assert_eq!(kill_note(-3), b"the kernel refused to end it");
    assert_ne!(kill_note(-1), kill_note(0));
}

#[test]
fn core_processes_are_protected_by_the_name_the_kernel_gives_them() {
    for name in [&b"compositor"[..], b"wm", b"desktop_shell", b"net.core", b"vfs_pool", b"init"] {
        assert!(protected(name, 40, 99), "{}", String::from_utf8_lossy(name));
    }
    assert!(!protected(b"app.calculator", 40, 99));
}

#[test]
fn this_window_is_protected_by_its_pid_and_other_windows_of_it_are_not() {
    assert!(protected(b"app.process_manager", 99, 99));
    assert!(!protected(b"app.process_manager.1", 41, 99));
    assert!(!protected(b"app.process_manager", 41, 99));
    assert!(!protected(b"app.process_manager", 41, 0));
}

#[test]
fn a_percent_has_no_decimal_the_kernel_did_not_report() {
    let mut buf = [0u8; 8];
    let n = percent(11, &mut buf);
    assert_eq!(&buf[..n], b"11%");
    let n = percent(0, &mut buf);
    assert_eq!(&buf[..n], b"0%");
}
