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

//! What sending a window's process the focus frame did, read without
//! sending into the inbox of a process that has ended.

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
