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

//! What an eject's firmware half is given, and how it ends.

/// What firmware gives the device being ejected.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Has {
    pub ej0: bool,
    pub removable: bool,
    pub lck: bool,
    pub dock: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Released {
    /// `_EJ0` ran; `_STA` says when firmware is done.
    ByFirmware,
    /// No `_EJ0`, `_RMV` says removable: the user takes it out.
    ForTheUser,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    /// Neither `_EJ0` nor a removable `_RMV`: nothing may be let go.
    NotRemovable,
    Unlock,
    Undock,
    Eject,
}

impl Stopped {
    /// The method that stopped it, for the log.
    pub fn method(self) -> &'static [u8] {
        match self {
            Stopped::NotRemovable => b"_EJ0 or _RMV, neither there",
            Stopped::Unlock => b"_LCK(0)",
            Stopped::Undock => b"_DCK(0)",
            Stopped::Eject => b"_EJ0",
        }
    }
}
