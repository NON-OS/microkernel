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

//! Which ends take a guest's window down, and in what order. Pure, so the
//! host proofs hold it.
//!
//! A guest's window was never taken down. Its toplevel destroyed, its
//! surface destroyed, its connection closed or its process ended, the
//! compositor went on drawing the last frame from memory this capsule had
//! let go, and the window manager kept the window: a ghost on the desktop
//! that a press still went to, with the dock hidden for good if it had
//! been full screen.

/// What ended.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum End {
    /// xdg_toplevel.destroy of this toplevel.
    Toplevel(u32),
    /// wl_surface.destroy of this surface.
    Surface(u32),
}

/// The surface the shown window's pixels come from, and its toplevel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shown {
    pub surface: u32,
    pub toplevel: Option<u32>,
}

/// Whether `end` takes down the window `shown`. A closed connection or an
/// ended process takes down whatever window it had (close_window.rs).
pub fn takes_down(end: End, shown: Option<Shown>) -> bool {
    let Some(s) = shown else { return false };
    match end {
        End::Toplevel(t) => s.toplevel == Some(t),
        End::Surface(id) => s.surface == id,
    }
}

/// Whether closing descriptor `fd` closes the display connection, given
/// which of the process's descriptors are connected display sockets: `fd`
/// is one, and no other is. A dup keeps the connection open.
pub fn ends_connection(connected: &[bool], fd: usize) -> bool {
    connected.get(fd) == Some(&true)
        && connected.iter().enumerate().all(|(i, &c)| i == fd || !c)
}

/// The calls a window is taken down with.
pub trait Steps {
    /// The compositor stops drawing the window.
    fn scene_remove(&mut self) -> bool;
    /// The kernel lets go of the surface over the pixels.
    fn release_surface(&mut self) -> bool;
    /// The pixels go back to the heap.
    fn free_pixels(&mut self);
    /// The pixels are kept for good, since the kernel may still map them.
    fn keep_pixels(&mut self);
    /// The window manager forgets the window, which brings the dock back.
    fn wm_close(&mut self) -> bool;
}

/// Take the window down. Every step is tried whatever the others did, as a
/// native app's are (app_skeleton runner/teardown_steps.rs), except that the
/// pixels are freed only once the surface over them is released. True when
/// every step went through.
pub fn take_down(steps: &mut impl Steps) -> bool {
    let removed = steps.scene_remove();
    let released = steps.release_surface();
    if released {
        steps.free_pixels();
    } else {
        steps.keep_pixels();
    }
    let closed = steps.wm_close();
    removed && released && closed
}
