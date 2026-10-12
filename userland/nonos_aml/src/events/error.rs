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

//! Why the event layer could not start or serve an SCI.

use super::fadt::FadtError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsError {
    Fadt(FadtError),
    /// A hardware-reduced platform has no fixed event blocks and no SCI
    /// (ACPI 6.5, 4.1); its events come through GPIO and the Generic Event
    /// Device instead.
    HwReduced,
    /// No PM1a event or control block.
    NoPm1,
    /// GPE0 and GPE1 claim some of the same event numbers (5.2.9 forbids it).
    Overlap,
    /// A register access the kernel refused or that failed.
    Io,
    /// SCI_EN did not come on within the wait after ACPI_ENABLE.
    ModeTimeout,
    /// Not in ACPI mode and no SMI command to switch with.
    NoModeSwitch,
}
