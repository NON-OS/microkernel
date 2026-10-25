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

//! The embedded controller's two ports and what they carry (ACPI 6.5,
//! 12.2): a status register read from the command port, commands written to
//! it, and bytes moved through the data port.

/// Status register bits (12.2.1).
pub const OBF: u16 = 1 << 0;
pub const IBF: u16 = 1 << 1;
pub const BURST: u16 = 1 << 4;
pub const SCI_EVT: u16 = 1 << 5;

/// Commands (12.3).
pub const RD_EC: u16 = 0x80;
pub const WR_EC: u16 = 0x81;
pub const QR_EC: u16 = 0x84;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EcPorts {
    pub data: u16,
    /// Status when read, command when written.
    pub command: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcError {
    /// The EC did not take a byte, or offer one, within the wait.
    Timeout,
    /// A port access the kernel refused or that failed.
    Io,
}
