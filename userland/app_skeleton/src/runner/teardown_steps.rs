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

//! The order a closed window is taken down in, and which step waits on
//! which. Pure, so a host proof drives it; teardown.rs gives it the calls.
//!
//! Every step stopped the ones after it when it failed, and the window
//! manager's close came last. A scene remove the compositor answered late,
//! which it does whenever a frame it is composing whole runs past the call's
//! budget, so returned before the window manager heard of the close: it kept
//! the window, a press over its old place still went to this app (idle by
//! then, or gone), the dock kept the app marked running, and opening it
//! again got the old window's rect back. Now each step is tried whatever
//! the others did, except the unmap, which waits on the surface release so
//! the compositor is never left reading memory given back.

pub trait Steps {
    fn scene_remove(&mut self) -> bool;
    fn unsubscribe_input(&mut self);
    fn release_surface(&mut self) -> bool;
    fn unmap_backing(&mut self) -> bool;
    fn wm_close(&mut self) -> bool;
}

/// Take the window down. True when every step went through.
pub fn take_down(steps: &mut impl Steps) -> bool {
    let removed = steps.scene_remove();
    steps.unsubscribe_input();
    let released = steps.release_surface();
    let unmapped = released && steps.unmap_backing();
    let closed = steps.wm_close();
    removed && unmapped && closed
}
