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

//! Where the battery client is, and the Battery Levels it found. It has a
//! phase of its own because it runs while the HID host is `Ready`.

/// One Battery Level characteristic: its value handle, the last handle of
/// its characteristic (where its descriptors end), and its properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Cell {
    pub value: u16,
    pub end: u16,
    pub props: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Charge {
    /// The HID reports are not on yet.
    Idle,
    /// Finding Battery Services from this handle on.
    Services { next: u16 },
    /// Finding one Battery Service's characteristics.
    Chars { service: usize, next: u16 },
    /// Reading one Battery Level (Vol 3, Part G, 4.8.1).
    Read { cell: usize },
    /// Finding one Battery Level's CCCD.
    Descs { cell: usize, next: u16 },
    /// Turning on one Battery Level's notifications.
    Enable { cell: usize },
    /// Every level found has been read; notifications keep them current.
    Done,
}
