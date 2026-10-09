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

//! The words the monitor uses when it has no row to draw or an action to
//! report. A table the kernel would not hand over, one still being read and a
//! filter that matches nothing all leave the table card blank, and a kill the
//! kernel refused looks exactly like one never sent, so the window names each.

/// `State::status` while the first read is outstanding, after a read the
/// kernel answered, and after one it refused.
pub const READING: &[u8] = b"reading process table";
pub const LIVE: &[u8] = b"live from the kernel";
pub const UNAVAILABLE: &[u8] = b"process table unavailable: the kernel refused the read";

/// The status strip's note when there is nothing to report: where the keys
/// are listed.
pub const KEYS_HINT: &[u8] = b"? keys";

/// What the table card says in place of rows, or `None` when it has rows to
/// draw. `total` is every row read, `shown` the ones the filter and search keep.
pub fn empty_table(total: usize, shown: usize, status: &'static [u8]) -> Option<&'static [u8]> {
    if shown != 0 {
        return None;
    }
    if total != 0 {
        return Some(b"No process matches the current filter or search");
    }
    if status == LIVE {
        return Some(b"The kernel listed no processes");
    }
    Some(status)
}

/// The note at the right end of the status strip: the last action's prompt or
/// outcome while it still applies, else a table that could not be read, else
/// where the keys are listed.
pub fn strip_note(notice: &'static [u8], status: &'static [u8]) -> &'static [u8] {
    if !notice.is_empty() {
        return notice;
    }
    if status == UNAVAILABLE {
        return status;
    }
    KEYS_HINT
}

/// End Process with nothing selected, on a protected process, and armed.
pub const NO_SELECTION: &[u8] = b"select a process first";
pub const PROTECTED: &[u8] =
    b"protected: a core system process or this window cannot be ended here";
pub const ARMED: &[u8] = b"press End Process or k again to confirm";

/// What the kernel's answer to an end means: it ended the process (or it had
/// already gone), it refused this window the authority, or it refused the
/// request itself. Values are the kernel's errnos (`MkKill`).
pub fn kill_note(rc: i64) -> &'static [u8] {
    match rc {
        0.. => b"ended",
        -1 => b"denied by the kernel: no authority over that process",
        -22 => b"the kernel refused the request as invalid",
        _ => b"the kernel refused to end it",
    }
}
