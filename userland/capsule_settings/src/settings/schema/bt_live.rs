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

//! The Bluetooth page's live rows, read from what driver.bt_usb0 last said
//! (`crate::bluetooth::panel`).

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BtLive {
    /// Whether the capsule found and brought up a USB adapter.
    Controller,
    /// The link: looking, connecting, securing or connected.
    Status,
    Scanning,
    /// The pairing window, and its seconds left.
    Window,
    /// What pairing asks of the user now.
    Asked,
}
