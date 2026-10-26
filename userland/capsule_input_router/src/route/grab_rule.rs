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
//! Over a focused window the shell may also grab while the user's last
//! press or key went to it (route/recent.rs): the menu bar's or the desktop's
//! menu, opened by that press, takes Esc and the click that closes it, and
//! the window keeps its focus. The trusted band's holder is decided first
//! (server/handlers/grab_request.rs), then focus, then that recency.

use crate::clients::wm;
use crate::state::Context;

/// The names that may grab, each one the image's own (the kernel lets no
/// capsule take one at run time). The installer holds a grab on an install
/// boot, where it is the whole screen.
pub const GRABBERS: [&[u8]; 5] =
    [b"app.boot_splash", b"app.setup_wizard", b"app.input_probe", b"desktop_shell", b"app.install"];

/// The decision, from the window manager's answer: Some(pid) for the focused
/// window's process, Some(0) for none, None for no answer. `last` is the
/// pid the router last delivered a press or key-down to.
pub fn may_grab(sender: u32, named: bool, focus: Option<u32>, shell: u32, last: u32) -> bool {
    let focused = match focus {
        None => true,
        Some(0) => sender == shell,
        Some(pid) => sender == pid || (sender == shell && last == shell),
    };
    sender != 0 && named && focused
}

/// The decision for `sender` now, asking the window manager.
pub fn grab_allowed(ctx: &mut Context, sender: u32, named: bool) -> bool {
    let rid = ctx.issue_request_id();
    let focus = wm::query_focus(&mut ctx.wm_port, rid);
    let shell = super::pointer::shell_pid(ctx);
    may_grab(sender, named, focus, shell, ctx.last_input)
}
