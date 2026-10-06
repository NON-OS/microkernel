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

use nonos_libc::mk_service_lookup;

const DESKTOP_SHELL: &[u8] = b"desktop_shell";

/// The desktop shell's pid, or 0 when it is not registered. Looked up at each
/// hit test rather than kept: a restarted shell has a new pid, and a stale one
/// would hand the dock's band to whatever process reused it.
pub(super) fn shell_pid() -> u32 {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(DESKTOP_SHELL.as_ptr(), DESKTOP_SHELL.len(), &mut port, &mut pid);
    if rc >= 0 && port != 0 {
        pid
    } else {
        0
    }
}
