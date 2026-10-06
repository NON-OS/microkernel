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

/// Why a firmware pin has no register to read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinError {
    /// The GPIO controller's `_HID` is not a layout this crate knows.
    UnknownController,
    /// No pad group of the layout carries that pin number.
    NotMapped,
}

/// Why a located pad cannot be read in the window that was mapped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadError {
    /// The window reads all ones or a zero pad base: nothing decodes there.
    Absent,
    /// The pad register falls past the end of the window.
    OutsideWindow,
}

impl PinError {
    pub fn why(self) -> &'static str {
        match self {
            PinError::UnknownController => "GPIO controller layout not known",
            PinError::NotMapped => "pin is in no pad group of the layout",
        }
    }
}

impl PadError {
    pub fn why(self) -> &'static str {
        match self {
            PadError::Absent => "GPIO community window reads as absent",
            PadError::OutsideWindow => "pad register lies past the community window",
        }
    }
}
