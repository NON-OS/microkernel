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

//! A capsule answering a kernel round trip on its reply port (send.rs).

extern crate alloc;

use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox;

/*
 * A capsule replying to a kernel-mediated round trip (crypto_pool,
 * entropy, vfs, the block device). The kernel drains this exact reply
 * inbox, so put the bytes there directly. Routing it as addressed would
 * send it to proc.<self>, where the serve loop reads its own reply as a
 * request and self-mails a core to death; that is the loop the old drop
 * guarded, and dropping instead stranded every kernel round trip.
 */
pub(super) fn to_reply_inbox(pid: u32, target: &str, data: &[u8]) -> i64 {
    if let Ok(msg) = IpcMessage::new(&alloc::format!("proc.{}", pid), target, data) {
        if nonos_inbox::try_enqueue_strict(target, msg).is_ok() {
            nonos_inbox::wake_waiter(target);
        }
    }
    0
}
