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

//! What one read of the input register brought.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LastRead {
    /// No answer, or less than the two-byte length.
    #[default]
    Failed,
    /// The zero-length report: nothing pending, or, read when the interrupt
    /// line said a report waited, the one a device serves once it has reset
    /// (HID over I2C 1.0, 7.2.1).
    Empty,
    Report,
}
