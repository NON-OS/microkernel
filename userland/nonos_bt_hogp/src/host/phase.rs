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

//! Where the host is.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Finding HID services from this handle on (Vol 3, Part G, 4.4.2).
    Services {
        next: u16,
    },
    /// Finding the characteristics of one service (Vol 3, Part G, 4.6.1).
    Chars {
        service: usize,
        next: u16,
    },
    /// Finding one source's CCCD (Vol 3, Part G, 4.7.1).
    Descs {
        source: usize,
        next: u16,
    },
    /// Turning on one source's notifications (Vol 3, Part G, 4.12.3).
    Enable {
        source: usize,
    },
    Ready,
    Failed,
}
