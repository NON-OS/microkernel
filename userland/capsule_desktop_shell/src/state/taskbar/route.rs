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

//! Where a click on an app that is marked running goes. The dock, the menus
//! and Open With raise a window the window manager said is open, newest
//! first, and open a new one only when the app has none.
//!
//! The dock sent the focus frame to the newest window's process and, when
//! that process was gone, to any live instance the registry named, window or
//! not; when neither took it, it said the app did not open and spawned
//! nothing. A window whose close never reached the shell (a crash, a close
//! the window manager answered late, a lost notice) so left the dock pointing
//! at a process that was gone: every click said the app did not open, or
//! handed the open to a process with no window to raise. Now each window is
//! asked in turn, a window whose process is gone is forgotten as its close
//! would have been, and an app left with no window is opened afresh.

use super::types::TaskbarState;
use super::windows::track_window_closed;

/// What sending a window's process the focus frame did.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reach {
    /// The frame was taken: the process restores, raises and focuses it.
    Taken,
    /// The process is there but its inbox is full: it is alive, with its
    /// window, and busy. The window is kept.
    Busy,
    /// No process takes frames under that pid any more.
    Gone,
}

/// The kernel's answer to a send into a full inbox.
pub const ERRNO_BUSY: i64 = -16;

/// What sending `pid` the focus frame means for its window: `alive` is the
/// kernel's MkPidAlive, `send` delivers the frame and returns the kernel's
/// answer.
///
/// A process that has ended keeps its `proc.<pid>` inbox until its tables
/// are freed, and the kernel takes a send into that inbox while the zombie
/// is still in the process table. The send alone so read an app that had
/// just ended as Taken: the dock click raised nothing and opened nothing,
/// and only the next click, after the runner's sweep, opened the app.
/// MkPidAlive reads a zombie as gone, so it is asked first, and a process
/// that is not alive is Gone without a frame sent into its inbox.
pub fn reach_by(pid: u32, alive: impl FnOnce(u32) -> bool, send: impl FnOnce(u32) -> i64) -> Reach {
    if !alive(pid) {
        return Reach::Gone;
    }
    match send(pid) {
        rc if rc >= 0 => Reach::Taken,
        ERRNO_BUSY => Reach::Busy,
        _ => Reach::Gone,
    }
}

/// Raise app `index`'s newest window whose process `reach` gets to. Each
/// window whose process is gone is forgotten on the way, so the running
/// mark goes out with the last of them. The pid raised, or None when the
/// app has no window left and must be opened.
pub fn raise_tracked(
    state: &mut TaskbarState,
    index: usize,
    mut reach: impl FnMut(u32) -> Reach,
) -> Option<u32> {
    loop {
        let w = state.windows.iter().rev().find(|w| w.app as usize == index).copied()?;
        match reach(w.pid) {
            Reach::Taken | Reach::Busy => return Some(w.pid),
            Reach::Gone => {
                let _ = track_window_closed(state, w.pid, w.window_id);
            }
        }
    }
}
