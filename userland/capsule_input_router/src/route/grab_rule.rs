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

//! Who may grab input (G, focus-only input): a capsule the router names
//! (the shell's inline rename, the splash, setup, the input probe, the
//! installer), and only while it has focus. A grab takes every key from the
//! focused window, so a named capsule in the background must not take them
//! from the window the user is typing into. With no window focused the
//! desktop is, and the shell may grab (a rename on the desktop). With no
//! window manager answering there is no focus to keep (setup and the
//! installer run before or without one), and the name alone decides.

use crate::clients::wm;
use crate::state::Context;

/// The decision, from the window manager's answer: Some(pid) for the focused
/// window's process, Some(0) for none, None for no answer.
pub fn may_grab(sender: u32, named: bool, focus: Option<u32>, shell: u32) -> bool {
    let focused = match focus {
        None => true,
        Some(0) => sender == shell,
        Some(pid) => sender == pid,
    };
    sender != 0 && named && focused
}

/// The decision for `sender` now, asking the window manager.
pub fn grab_allowed(ctx: &mut Context, sender: u32, named: bool) -> bool {
    let rid = ctx.issue_request_id();
    let focus = wm::query_focus(&mut ctx.wm_port, rid);
    let shell = super::pointer::shell_pid(ctx);
    may_grab(sender, named, focus, shell)
}
