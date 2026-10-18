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

//! Telling a holder its device left (DGON, `dgon_wire`): one message into
//! the inbox it reads requests on (`proc.<pid>`), from sender 0, after the
//! revoke and before the depart answers. Best effort: a holder whose inbox
//! is missing or full gets nothing, and that is counted and said.

use super::dgon_wire::{dgon_encode, Dgon, WHY_GONE};
use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox::try_enqueue_strict;
use crate::sys::serial::Line;
use core::sync::atomic::{AtomicU64, Ordering};

static UNDELIVERED: AtomicU64 = AtomicU64::new(0);

/// A sender name with no `proc.` prefix reads as sender 0 to the receiver.
const FROM: &str = "kernel.broker";

pub(super) fn notify_gone(pid: u32, device_id: u64) {
    let bytes = dgon_encode(Dgon { why: WHY_GONE, device_id });
    let to = alloc::format!("proc.{}", pid);
    let sent = IpcMessage::new(FROM, &to, &bytes).ok().map(|m| try_enqueue_strict(&to, m));
    if matches!(sent, Some(Ok(()))) {
        crate::sched::wake_process(pid);
        return;
    }
    let n = UNDELIVERED.fetch_add(1, Ordering::Relaxed) + 1;
    let mut l = Line::new();
    l.str(b"[ACPI] DGON to pid ").dec(u64::from(pid)).str(b" for device=").hex(device_id);
    l.str(b" not delivered (").dec(n).str(b" so far)").end();
}
