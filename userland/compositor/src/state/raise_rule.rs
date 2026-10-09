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

//! Who may raise a layer.
//!
//! OP_FOCUS_SET puts the target pid's layer on top of its band. The window
//! manager owns the stacking order and sends it whenever a window is focused
//! or raised there, so the two orders stay one. Nothing checked the sender,
//! so any client could put its own window over another's at will, over a
//! prompt asking for a password among them, while the window manager went on
//! routing input to the window it had focused. Only the window manager's
//! current pid may raise a layer now; a client gets E_PERM.

/// Whether `sender_pid` may raise a layer, given the pid the service
/// registry names for the window manager (`None` while it is not running).
pub fn may_raise(sender_pid: u32, wm_pid: Option<u32>) -> bool {
    sender_pid != 0 && wm_pid == Some(sender_pid)
}
