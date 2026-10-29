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

//! What one look of MkForeignWait finds, pure. Held in kernel_proofs.

/// What a look finds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Look {
    /// A guest trapped, or one died: its frame is handed over.
    Frame,
    /// A message came to the caller's own inbox since it was last told: the
    /// wait returns 0, with nothing written.
    Inbox,
    /// Neither: sleep until one, or the timeout.
    Sleep,
}

/// One look: `trapped` when a guest's frame waits; `enqueued` messages have
/// been put in the caller's own inbox in all, and it was last told when there
/// were `heard`, 0 before. Told once a count, a message the caller leaves
/// unread does not keep waking it; a frame goes first, the inbox next look.
pub(crate) const fn look(trapped: bool, enqueued: u64, heard: u64) -> Look {
    if trapped {
        Look::Frame
    } else if enqueued != heard {
        Look::Inbox
    } else {
        Look::Sleep
    }
}
