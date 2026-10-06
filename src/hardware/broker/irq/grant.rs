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

//! What the broker remembers about one bound interrupt vector.

/// The line a message-signalled slot records: no IO-APIC pin carries it, so
/// the dispatcher has nothing to mask and the ack nothing to unmask.
pub(super) const NO_LINE: u32 = u32::MAX;

/// The remapping entry of a grant whose message is in compatibility format.
/// Entry 0 is never handed out by the IOMMU, so it cannot name a real one.
pub(super) const NO_IRTE: u16 = 0;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqGrantKind {
    Intx = 0,
    Msix = 1,
    Msi = 2,
}

#[derive(Debug, Clone, Copy)]
pub struct IrqGrant {
    pub grant_id: u64,
    pub pid: u32,
    pub device_id: u64,
    pub claim_epoch: u64,
    pub irq_source: u32,
    pub vector: u8,
    pub flags: u32,
    pub kind: IrqGrantKind,
    // For `IrqGrantKind::Msix` this is the index of the MSI-X table
    // entry the kernel programmed for this grant (0..table_size).
    // Always 0 for INTx and MSI grants.
    pub device_vector: u16,
    // The VT-d interrupt remapping entry behind an MSI or MSI-X grant,
    // `NO_IRTE` when its message names a LAPIC directly.
    pub irte: u16,
}
