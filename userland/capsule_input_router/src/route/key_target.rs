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

//! Who a key press goes to.
//!
//! `answer` is the window manager's: Some(pid) for the process whose window
//! has focus, Some(0) when no window has it (the last one closed or was
//! minimised), None when it did not answer at all. Only that last case falls
//! back to the process the previous key went to.
//!
//! The two used to be one None, so with every window closed or minimised the
//! keys went on to the last window's process: a minimised terminal kept
//! taking what was typed with nothing on screen showing it. With no window
//! focused the keys go to the desktop shell, as a press on no window does.

pub fn press_target(answer: Option<u32>, last_focus_pid: u32, shell_pid: u32) -> u32 {
    match answer {
        Some(0) => shell_pid,
        Some(pid) => pid,
        None if last_focus_pid != 0 => last_focus_pid,
        None => shell_pid,
    }
}
