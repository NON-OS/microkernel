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

//! A capsule's output line, mirrored into its own `proc.<pid>` inbox.

/*
 * Mirror the line into the calling process's own `proc.<pid>` inbox so a
 * launcher (the terminal) can drain a child capsule's stdout into its
 * window. Best effort: a missing or full inbox is ignored, and serial
 * stays the source of truth for trust logs.
 */
pub(in crate::syscall::microkernel) fn mirror_to_proc_inbox(bytes: &[u8]) {
    let Some(pid) = crate::process::current_pid() else {
        return;
    };
    let name = alloc::format!("proc.{}", pid);
    /*
     * Skip the copy when the inbox is missing or already full. Nothing is
     * draining most capsules, so their inbox fills once and then every later
     * line is dropped here without building a message.
     */
    if !crate::ipc::nonos_inbox::exists(&name) || crate::ipc::nonos_inbox::is_full(&name) {
        return;
    }
    if let Ok(msg) = crate::ipc::nonos_channel::IpcMessage::new(&name, &name, bytes) {
        let _ = crate::ipc::nonos_inbox::try_enqueue_strict(&name, msg);
    }
}
