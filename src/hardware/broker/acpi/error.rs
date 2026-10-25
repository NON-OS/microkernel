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

//! Why an ACPI region request failed, for the syscall layer's errno.

use super::rule::Refusal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionError {
    /// The grant rule said no; the refusal was logged.
    Refused(Refusal),
    /// The capsule holds as many windows as the kernel will record.
    Full,
    /// The MMIO window of the capsule's address space is used up.
    NoVaSpace,
    MapFailed,
    /// An access width other than 1, 2 or 4 bytes, or one not aligned to it.
    BadWidth,
    /// A port or function access outside every window the caller holds.
    NotGranted,
    /// A PCI segment other than 0, which the kernel does not enumerate.
    Unsupported,
    /// The configuration access itself failed.
    Platform,
    /// A port write that would set SLP_EN, the kernel's alone (`sleep_enable`).
    SleepEnable,
}
