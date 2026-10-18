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

//! Section tags, and the length each may have.

use crate::delivery::DELIVERY_BYTES;
use crate::record::RECORD_BYTES;

pub const RECORD: u16 = 1;
pub const DEVICE: u16 = 2;
pub const ISOLATION: u16 = 3;
pub const PUBLICS: u16 = 4;
pub const ATTEST: u16 = 5;
pub const DELIVERY: u16 = 6;
pub const MEASUREMENTS: u16 = 7;
pub const POLICY: u16 = 8;

/// The highest tag this version knows.
pub const LAST: u16 = POLICY;

/// The isolation statement's public words, Hc and A: eight little-endian u64s.
pub const PUBLICS_BYTES: usize = 64;
pub const POLICY_BYTES: usize = 256;

/// Whether `len` bytes can be section `tag`. Fixed sections have their exact
/// length; a list is whole entries; a proof is anything but empty.
pub fn fits(tag: u16, len: usize) -> bool {
    match tag {
        RECORD => len == RECORD_BYTES,
        PUBLICS => len == PUBLICS_BYTES,
        DELIVERY => len == DELIVERY_BYTES,
        POLICY => len == POLICY_BYTES,
        MEASUREMENTS => len > 0 && len.is_multiple_of(32),
        DEVICE | ISOLATION | ATTEST => len > 0,
        _ => false,
    }
}
