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

//! MkAcpiRestoreFunction's answer before anything is written, pure (L64):
//! a function the broker knows, claimed, and in D0, since only then do its
//! BARs answer and can be put back. Device id 0 names nothing.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Marf {
    Unknown,
    NotClaimed,
    NotInD0,
}

/// `pm`: the function's power management control and status register,
/// None when it has no power management capability and so is always in D0.
pub fn check(device_id: u64, known: bool, claimed: bool, pm: Option<u16>) -> Result<(), Marf> {
    if device_id == 0 || !known {
        Err(Marf::Unknown)
    } else if !claimed {
        Err(Marf::NotClaimed)
    } else if pm.is_some_and(|p| p & 0x3 != 0) {
        Err(Marf::NotInD0)
    } else {
        Ok(())
    }
}
