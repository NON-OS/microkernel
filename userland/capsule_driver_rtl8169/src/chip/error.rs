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

use super::Chip;

#[derive(Debug, PartialEq, Eq)]
pub enum ChipError {
    /// TxConfig read back all ones: the BAR does not answer.
    ReadFailed,
    /// An XID (or extended id) Linux has no row for.
    Unknown { xid: u32, extended: bool },
    /// The extended id register lies past the window the broker mapped.
    ExtendedOutOfWindow,
    /// An 8125 whose mapped window does not reach the registers its start
    /// writes (up to Q_NUM_CTRL_8125 at 0x4800); the length mapped.
    BarTooSmall(u64),
    /// A chip Linux knows that this driver does not start: the 5G RTL8126A
    /// and the 10G RTL8127A, whose bring-up differs from the 8125's.
    Unsupported(Chip),
}

impl ChipError {
    /// The reason `start_driver` prints after "bring-up failed".
    pub fn as_str(&self) -> &'static str {
        match self {
            ChipError::ReadFailed => "rtl8169 TxConfig reads all ones",
            ChipError::Unknown { .. } => "rtl8169 unknown chip xid",
            ChipError::ExtendedOutOfWindow => "rtl8169 extended chip id outside the bar",
            ChipError::BarTooSmall(_) => "rtl8169 8125 bar too small",
            ChipError::Unsupported(_) => "rtl8169 chip not supported",
        }
    }
}
