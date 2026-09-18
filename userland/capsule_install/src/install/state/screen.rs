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

//! The six screens, in the order a person meets them. Failed can follow
//! Writing or Verifying; nothing else is out of order.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Welcome,
    Disks,
    Confirm,
    Writing,
    Verifying,
    Done,
    Failed,
}

impl Screen {
    pub fn title(self) -> &'static str {
        match self {
            Screen::Welcome => "Install NØNOS on this computer",
            Screen::Disks => "Choose the disk",
            Screen::Confirm => "Everything on this disk will be erased",
            Screen::Writing => "Writing",
            Screen::Verifying => "Reading back",
            Screen::Done => "Installed",
            Screen::Failed => "Stopped",
        }
    }

    /// One-based step for the header, of five that a person acts in.
    pub fn step(self) -> u8 {
        match self {
            Screen::Welcome => 1,
            Screen::Disks => 2,
            Screen::Confirm => 3,
            Screen::Writing | Screen::Verifying | Screen::Failed => 4,
            Screen::Done => 5,
        }
    }
}
