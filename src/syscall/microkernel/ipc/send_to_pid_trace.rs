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

//! MkIpcSendToPid's serial trace, for the pid under watch.

pub(super) fn trace(caller_pid: u32, dest_pid: u64, len: usize) {
    if caller_pid != 0x17 {
        return;
    }
    crate::sys::serial::trace(b"[IPC-RPLY] from=");
    crate::sys::serial::trace_hex(caller_pid as u64);
    crate::sys::serial::trace(b" to=");
    crate::sys::serial::trace_hex(dest_pid);
    crate::sys::serial::trace(b" len=");
    crate::sys::serial::trace_dec(len as u64);
    crate::sys::serial::traceln(b"");
}
