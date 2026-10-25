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

//! A program this terminal loaded that draws a window waits, before it opens
//! one, for a focus frame it takes from the desktop shell alone. So the
//! terminal asks the shell to send it (OP_FOCUS_CHILD); the shell does only
//! for the asker's own child, and only for a program holding a display right
//! (desktop_shell's state/apps/focus_rule.rs). A program with no window is
//! refused there, and the terminal drains its output as before.

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::call_status;

// Hand-synced with desktop_shell's protocol::{MAGIC, OP_FOCUS_CHILD}.
const NDSH: u32 = 0x4E44_5348;
const OP_FOCUS_CHILD: u16 = 0x000F;

/// Whether `pid` now opens its own window.
pub(super) fn open(pid: u32) -> bool {
    let Some(shell) = lookup_service(b"desktop_shell") else {
        return false;
    };
    matches!(call_status(shell.port, NDSH, OP_FOCUS_CHILD, 1, &pid.to_le_bytes()), Ok(0))
}
