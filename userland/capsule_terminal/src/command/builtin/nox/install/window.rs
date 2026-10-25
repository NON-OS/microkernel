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
//! terminal asks the shell to send it (OP_FOCUS_CHILD), which the shell does
//! only for the asker's own child (desktop_shell's state/apps/focus_rule.rs).
//! Only a program that draws (GraphicsSurfaceCreate, read from the process
//! table, whose caps the terminal sees by AttestRead) is asked for; any other
//! has its output drained as before.

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::call_status;

use crate::command::builtin::proc_table::table;

// Hand-synced with desktop_shell's protocol::{MAGIC, OP_FOCUS_CHILD}.
const NDSH: u32 = 0x4E44_5348;
const OP_FOCUS_CHILD: u16 = 0x000F;

/// GraphicsSurfaceCreate: a program without it has no window to open.
const DRAWS: u64 = 1 << 12;

/// Whether `pid` now opens its own window.
pub(super) fn open(pid: u32) -> bool {
    let draws = table().iter().any(|e| e.pid == pid && e.caps & DRAWS != 0);
    let Some(shell) = lookup_service(b"desktop_shell").filter(|_| draws) else {
        return false;
    };
    matches!(call_status(shell.port, NDSH, OP_FOCUS_CHILD, 1, &pid.to_le_bytes()), Ok(0))
}
