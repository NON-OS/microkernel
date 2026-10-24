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

//! The values an x86 grant records for what it does not have. Only the
//! IO-APIC and VT-d paths that bind here have pins and remapping entries.

/// The line a message-signalled slot records: no IO-APIC pin carries it, so
/// the dispatcher has nothing to mask and the ack nothing to unmask.
pub(in crate::hardware::broker::irq) const NO_LINE: u32 = u32::MAX;

/// The remapping entry of a grant whose message is in compatibility format.
/// Entry 0 is never handed out by the IOMMU, so it cannot name a real one.
pub(in crate::hardware::broker::irq) const NO_IRTE: u16 = 0;
