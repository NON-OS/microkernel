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
//! What a probe of one address on the bus found.

/// What an address probe found.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Presence {
    /// Nothing acknowledged the address.
    Absent,
    /// A device acknowledged, but no register it was asked for read back as
    /// an HID descriptor: present, possibly still waking, possibly not HID.
    Acked,
    /// A 30-byte HID descriptor (wHIDDescLength 30, bcdVersion 1.00) came
    /// back from one of the registers tried.
    HidDescriptor(u16),
}
