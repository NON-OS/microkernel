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

//! What a launch that opened nothing says after the app's name.

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
