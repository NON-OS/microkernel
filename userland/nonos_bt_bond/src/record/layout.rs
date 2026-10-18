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

//! Where each field sits in a bond record. Little endian where a field has
//! an order of its own on the air; the LTK and IRK most significant octet
//! first, as the Security Manager's functions take them (Vol 3, Part H,
//! 2.2).
//!
//! ```text
//! [0]      version, 1
//! [1]      flags: bit 0 authenticated, bit 1 IRK present, bit 2 LE Secure
//!          Connections (always set); the rest zero
//! [2]      identity address type: 0 public, 1 static random
//! [3..9]   identity address, least significant octet first
//! [9..25]  LTK
//! [25..41] IRK, zero when absent
//! [41]     key size, 7 to 16
//! [42..48] zero
//! ```

pub const RECORD_LEN: usize = 48;
pub(super) const VERSION: u8 = 1;
pub(super) const AUTHENTICATED: u8 = 0x01;
pub(super) const HAS_IRK: u8 = 0x02;
pub(super) const SECURE_CONNECTIONS: u8 = 0x04;
pub(super) const KNOWN_FLAGS: u8 = 0x07;
pub(super) const ADDR: usize = 3;
pub(super) const LTK: usize = 9;
pub(super) const IRK: usize = 25;
pub(super) const KEY_SIZE: usize = 41;
pub(super) const RESERVED: usize = 42;
