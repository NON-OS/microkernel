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

extern crate alloc;

use alloc::vec::Vec;

use super::registry::REGISTRY;
use crate::ipc::nonos_channel::IpcMessage;

/// Take at most `max` bytes from the front of `module`, as a stream: the
/// rest of a longer message stays first in line for the next take, so no
/// byte is dropped. The kernel's copies of what was split are zeroed. The
/// registry is held for writing throughout, so no other reader or writer
/// sees the queue while it is put back together. `None` when the inbox is
/// missing or empty.
pub fn take_front(module: &str, max: usize) -> Option<Vec<u8>> {
    let reg = REGISTRY.write();
    let inbox = reg.map.get(module)?;
    let mut msg = inbox.dequeue()?;
    if msg.data.len() <= max {
        return Some(core::mem::take(&mut msg.data));
    }
    let head = msg.data[..max].to_vec();
    let rest = IpcMessage::with_timestamp(&msg.from, &msg.to, &msg.data[max..], msg.timestamp_ms);
    crate::crypto::secure_zero(&mut msg.data);
    let mut queued = Vec::new();
    while let Some(m) = inbox.dequeue() {
        queued.push(m);
    }
    /*
     * The queue held `msg` and `queued`, so the remainder and `queued` fit
     * back in, by count and by bytes: the registry is held for writing, so
     * no other enqueue anywhere took the bytes their dequeue gave back. A
     * remainder is only unbuildable without the IPC secret, which the
     * message itself was built with; should it fail, say so.
     */
    if rest.is_err() {
        crate::sys::serial::print(b"[INBOX] remainder of a split message lost\n");
    }
    for m in rest.ok().into_iter().chain(queued) {
        if let Err(mut lost) = inbox.try_enqueue(m) {
            crate::crypto::secure_zero(&mut lost.data);
        }
    }
    Some(head)
}
