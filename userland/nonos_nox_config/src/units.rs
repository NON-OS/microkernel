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

//! What a price is per: keccak256 of a lowercase label. A listing names one,
//! and a voucher and its usage leaves carry the same word.

use crate::hex::hex32;

pub const UNIT_INSTALL: [u8; 32] =
    hex32("a664b566d5222344936e9116a24fc8f22ccb3fe87041e4fcddb461709eb5c6f3");
pub const UNIT_PROOF: [u8; 32] =
    hex32("a5bd20250df117ee1576cde77471907f0792dabd126e96e46ea0b2c71299ea1e");
pub const UNIT_CALL: [u8; 32] =
    hex32("431a75a0d711dfe3639aae96eb8f9facc2fd74df5aa611067f2511cc9fefc229");
pub const UNIT_SECOND: [u8; 32] =
    hex32("45318970bfff215a328f56895f3a97d4f276a44c24c135c12c37867a1f667b8a");
pub const UNIT_BYTE: [u8; 32] =
    hex32("add4bfc7bb557d02d6c639d2a54f9300196b2fdb6b83d6d613d4cfd52221306f");

pub const UNITS: [(&str, [u8; 32]); 5] = [
    ("install", UNIT_INSTALL),
    ("proof", UNIT_PROOF),
    ("call", UNIT_CALL),
    ("second", UNIT_SECOND),
    ("byte", UNIT_BYTE),
];
