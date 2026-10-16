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

//! MkIpcReply's serial trace, for the two pids under watch.

pub(super) fn trace(caller_pid: u32, dest_pid: u64, label: &[u8], rc: i64, inbox: &str) {
    if caller_pid != 0x9 && dest_pid != 0x17 {
        return;
    }
    crate::sys::serial::trace(b"[IPC-REPLY] from=");
    crate::sys::serial::trace_hex(caller_pid as u64);
    crate::sys::serial::trace(b" to=");
    crate::sys::serial::trace_hex(dest_pid);
    crate::sys::serial::trace(b" inbox=");
    crate::sys::serial::trace(inbox.as_bytes());
    crate::sys::serial::trace(b" ");
    crate::sys::serial::trace(label);
    crate::sys::serial::trace(b" rc=");
    if rc < 0 {
        crate::sys::serial::trace(b"-");
        crate::sys::serial::trace_dec((-rc) as u64);
    } else {
        crate::sys::serial::trace_dec(rc as u64);
    }
    crate::sys::serial::traceln(b"");
}
