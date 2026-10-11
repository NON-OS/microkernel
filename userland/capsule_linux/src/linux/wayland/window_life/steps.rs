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

//! The steps a window is taken down by, in order. Pure, so the host proofs
//! hold it.

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
