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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmdViError {
    /// IVRS named no unit, or bring-up has not run.
    NotPresent,
    /// Units exist but are not translating with the kernel's tables.
    NotEnforcing,
    /// A unit did not store a completion wait within `COMMAND_MS`, or its
    /// command buffer stopped draining.
    Timeout,
    /// No frame, or no contiguous run, for a table.
    NoFrames,
    TableUnreachable,
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
}
