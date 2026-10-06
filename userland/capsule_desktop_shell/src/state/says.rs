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

//! The words the shell's toasts use when something did not happen, kept apart
//! from the IPC so each is proven on the host (desktop_proofs says_tests). A
//! toast holds `TOAST_TEXT_MAX` bytes; a line built here is cut on its name,
//! never on the part that says what went wrong.

/// `head`, then `tail`, in `out`: `head` cut so `tail` always fits whole.
pub fn named(head: &[u8], tail: &[u8], out: &mut [u8]) -> usize {
    let tail = &tail[..tail.len().min(out.len())];
    let n = head.len().min(out.len() - tail.len());
    out[..n].copy_from_slice(&head[..n]);
    out[n..n + tail.len()].copy_from_slice(tail);
    n + tail.len()
}

/// After an app's name: a launch was asked for and no window of the app
/// came in the wait state/taskbar/expect.rs gives it. The kernel's serial
/// log says why init spawned nothing, or the app why it opened nothing.
pub const NO_WINDOW: &[u8] = b" did not open: no window in 30 s";
/// After an app's name: setup turned it off.
pub const OFF_AT_SETUP: &[u8] = b" turned off at setup";

/// No spawn was asked for since the last launch was said.
pub const NOTHING_REFUSED: i64 = 0;
/// The app opens one window only, so no spawn was asked for.
pub const NOT_ASKED: i64 = i64::MIN;

/// After an app's name: why a launch opened nothing, from the kernel's
/// answer to the spawn (`NOT_ASKED` and `NOTHING_REFUSED` as above). Each
/// fits a toast whole after the longest dock name.
pub fn not_opened(refusal: i64) -> &'static [u8] {
    match refusal {
        NOTHING_REFUSED => b" did not open: its window is gone",
        /* One window only, and its process is gone: the kernel has no
         * instance of it to spawn (ENOENT) and nothing answered the open. */
        NOT_ASKED | -2 => b" is not running; it has one window",
        -13 => OFF_AT_SETUP,
        -16 => b" did not open: busy, try again",
        -1 => b" did not open: not permitted",
        -22 => b" did not open: bad app name",
        _ => b" did not open: the kernel refused",
    }
}

/// What the installer's answer to a package query or commit means, in the
/// words the Terminal's `pkg` uses, short enough for a toast after
/// "Package: ". Values are the installer's errnos.
pub fn package(code: i32) -> &'static [u8] {
    match code {
        -2 => b"not found",
        -5 => b"store write failed",
        -11 => b"installer not ready, try again",
        -13 => b"signature or digest failed verification",
        -17 => b"already installed",
        -22 => b"malformed package or path",
        -71 => b"malformed reply from installer",
        _ => b"the installer refused it",
    }
}

/// Why the file service refused a desktop action. Values are vfs_pool's
/// errnos; `NO_REPLY` is the shell's own for a call nothing answered.
pub const NO_REPLY: i32 = i32::MIN;

pub fn vfs(code: i32) -> &'static [u8] {
    match code {
        NO_REPLY => b"the file service did not answer",
        -2 => b"it is not there",
        -13 => b"access denied",
        -17 => b"that name is taken",
        -22 => b"not a valid name",
        -28 => b"no space left",
        -39 => b"the folder is not empty",
        _ => b"the file service refused",
    }
}

/// What the Launchpad shows when its search leaves no app, tool or package
/// standing: that nothing matched, and the two ways out. None while
/// something matches, or before anything is typed.
pub fn launchpad_empty(query: &str, matched: usize) -> Option<[&'static str; 2]> {
    if matched != 0 || query.is_empty() {
        return None;
    }
    Some(["No app, tool or package has that in its name.", "Backspace to change it, Esc to clear it."])
}
