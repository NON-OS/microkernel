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

//! One window of the guest's: a mapped toplevel, the NONOS surface its
//! pixels are on, where that is, its frame and its mode.

use super::fit::Fit;
use super::reshape::Shape;
use super::scene_buffers::Buffers;
use super::window_life::Shown;

#[derive(Default)]
pub struct Window {
    /// The window manager's id for the window, kept across new surfaces.
    pub window: Option<u32>,
    /// Whether the window has the keyboard, as the window manager last said
    /// (input/focus.rs): its toplevel is told `activated` while it does.
    pub focused: bool,
    /// The buffer the window's layer shows now, which names the layer to
    /// the compositor; it moves to the other buffer at each present.
    pub out: Option<u64>,
    /// Where the window manager placed that surface's window on screen.
    pub at: Option<(u32, u32)>,
    /// The width, height and stride that surface was registered at.
    pub shape: Option<Shape>,
    /// The surface's point at the window's corner, in its logical pixels:
    /// its window geometry's, or (0, 0) for a buffer shown whole
    /// (present/crop.rs). The pointer and what is placed on it count from it.
    pub origin: (i32, i32),
    /// The guest surface, and its toplevel, the window shows.
    pub shown: Option<Shown>,
    /// Full screen and back (fit.rs), and the toplevel that asked for it.
    pub fit: Fit,
    pub fit_of: Option<u32>,
    /// Hidden by the window manager: its layer is off the compositor and
    /// its buffers let go of, its window kept so a restore shows it again
    /// where it was (present/hide.rs). Its commits are not shown meanwhile.
    pub minimized: bool,
    /// The window's two buffers, one shown, one drawn (scene_buffers.rs).
    pub bufs: Buffers,
    /// A draw that found no buffer to draw in, or whose present the
    /// compositor refused, still to be shown (present/owed.rs).
    pub owing: Option<super::present::owed::Owing>,
    /// The NØNOS frame drawn around it (present/frame/).
    pub frame: super::present::frame::Frame,
    /// The window's own pixels before any subsurface was laid on them, kept
    /// while one shows, so a desynchronized commit repaints without them
    /// (subsurface/compose.rs).
    pub base: alloc::vec::Vec<u8>,
    /// The client's buffer the window's layer draws over its own, from its
    /// pin, while it shows one so (present/pinned/).
    pub content: Option<super::present::pinned::held::Held>,
    /// When its client last committed, kept from the time the window gave
    /// its pin back at idle until it shows one again: only a run of frames
    /// is pinned again (present/pinned/rule.rs paced).
    pub idle_back: Option<u64>,
    /// When its client last committed while it showed its own buffers, and
    /// how many commits in a row came sooner than RUN_MS after the one
    /// before: only a run of frames is pinned (present/pinned/rule.rs).
    pub own_last: Option<u64>,
    pub run: u32,
    /// The buffer its client committed last, and the other one it committed
    /// before that: a client drawing in two in turn has that one free to
    /// draw its next frame in (present/pinned/pins.rs spare).
    pub committed: Option<u32>,
    pub turn: Option<u32>,
}
