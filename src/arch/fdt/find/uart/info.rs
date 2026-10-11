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

use super::kind::UartKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UartInfo {
    pub kind: UartKind,
    pub base: u64,
    /// The input clock, when the tree states it as a plain number. `None`
    /// means the divisor firmware programmed is the only one known to work.
    pub clock_hz: Option<u32>,
    /// The line speed the tree asks for, if it asks.
    pub baud: Option<u32>,
    /// Registers sit `1 << reg_shift` bytes apart and are read and written
    /// `io_width` bytes at a time.
    pub reg_shift: u8,
    pub io_width: u8,
}
