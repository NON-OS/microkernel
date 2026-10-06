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

//! Where a key press goes, by the window manager's answer.

use crate::router_key_target::press_target;

const SHELL: u32 = 5;
const TERMINAL: u32 = 40;

#[test]
fn a_focused_window_takes_the_key() {
    assert_eq!(press_target(Some(TERMINAL), 0, SHELL), TERMINAL);
    assert_eq!(press_target(Some(TERMINAL), 77, SHELL), TERMINAL);
}

/// The terminal was the last window and was minimised: the window manager
/// says no window has focus. The key goes to the shell, not to the terminal
/// the previous key went to.
#[test]
fn no_focused_window_sends_the_key_to_the_shell_not_the_last_window() {
    assert_eq!(press_target(Some(0), TERMINAL, SHELL), SHELL);
}

#[test]
fn a_window_manager_that_did_not_answer_keeps_the_last_target() {
    assert_eq!(press_target(None, TERMINAL, SHELL), TERMINAL);
    assert_eq!(press_target(None, 0, SHELL), SHELL);
}
