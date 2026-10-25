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

use crate::arch::x86_64::iommu::tables::out_of_frame::OutOfFrame;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmdViError {
    /// IVRS named no unit, or bring-up has not run.
    NotPresent,
    /// Units exist but are not translating with the kernel's tables.
    NotEnforcing,
    /// A unit did not store a completion wait within `COMMAND_MS`, or its
    /// command buffer stopped draining.
    Timeout,
    /// A map's entries were written and the domain flush failed: the range
    /// may be live.
    Uncommitted,
    /// No frame, or no contiguous run, for a table.
    NoFrames,
    TableUnreachable,
    /// The frame allocator would not take a dead domain's table back; the
    /// tables not yet returned stay allocated, never reused.
    TableNotFreed,
    RegistersUnmappable,
    DomainNotFound,
    DomainTableFull,
    Misaligned,
    OutOfRange,
    RangeAlreadyMapped,
    RangeNotMapped,
    DeviceAlreadyAttached,
    DeviceNotAttached,
    /// A map granting neither read nor write would read as unmapped.
    NoPermissions,
    /// The device table held a writer entry with no valid translation before a
    /// unit was enabled; the unit is refused rather than let it through.
    StrayWriter,
    /// A device's remapping table, or the broker's handles, has no free entry.
    InterruptTableFull,
    /// The device's requests reach the unit under an IVHD alias id.
    Aliased,
    /// A failed map's leaves could not be made unreachable: the unit may
    /// still hold them, so the frames must never be reused.
    Stranded,
    /// A unit's EFR.HATS does not cover the four-level tables the kernel
    /// builds (Mode 4), or the register cannot be read (K12).
    DepthUnsupported,
}

impl From<OutOfFrame> for AmdViError {
    fn from(_: OutOfFrame) -> Self {
        AmdViError::OutOfRange
    }
}
