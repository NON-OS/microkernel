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
use super::types::Regs;

impl Regs {
    /// The doorbell of the queue whose queue_notify_off is `queue_notify`.
    /// Both factors come from the device; the product was unchecked and
    /// could put the doorbell write anywhere in the address space. It is now
    /// the shared checked arithmetic, and `None` unless the 16-bit write
    /// lands inside the mapped notify region on a 2-byte boundary.
    pub fn with_queue_notify(self, queue_notify: u16) -> Option<Self> {
        let multiplier = u32::try_from(self.notify_multiplier).ok()?;
        let off = nonos_virtio::notify::notify_offset(queue_notify, multiplier, self.notify_len)?;
        Some(Self { notify_offset: self.notify_offset.checked_add(off)?, ..self })
    }
}
