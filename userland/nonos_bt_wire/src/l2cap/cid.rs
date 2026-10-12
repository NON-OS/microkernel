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

//! LE channel identifiers (Vol 3, Part A, 2.1): the fixed channels an
//! LE-only host uses, and the range for credit-based dynamic channels.

/// The Attribute Protocol.
pub const ATT: u16 = 0x0004;
/// The LE L2CAP signaling channel.
pub const LE_SIGNALING: u16 = 0x0005;
/// The Security Manager Protocol.
pub const SMP: u16 = 0x0006;
/// The dynamically allocated LE channels.
pub const LE_DYNAMIC: core::ops::RangeInclusive<u16> = 0x0040..=0x007f;
