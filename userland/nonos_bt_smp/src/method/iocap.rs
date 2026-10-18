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

//! A device's IO capability as Pairing Request and Response carry it
//! (Vol 3, Part H, 3.5.1).

use nonos_bt_wire::WireError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoCap {
    DisplayOnly,
    DisplayYesNo,
    KeyboardOnly,
    NoInputNoOutput,
    KeyboardDisplay,
}

impl IoCap {
    pub fn from_wire(v: u8) -> Result<Self, WireError> {
        use IoCap::*;
        Ok(match v {
            0 => DisplayOnly,
            1 => DisplayYesNo,
            2 => KeyboardOnly,
            3 => NoInputNoOutput,
            4 => KeyboardDisplay,
            _ => return Err(WireError::Reserved),
        })
    }

    pub fn wire(self) -> u8 {
        self as u8
    }
}
