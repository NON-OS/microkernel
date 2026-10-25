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

//! Settle's BROADCAST as every wallet send asks it: the
//! body, chain id little-endian then the signed bytes, and the answer, the
//! transaction's 32-byte hash. Settle sends it over its own route.

use alloc::vec::Vec;

/// What BROADCAST is asked with: `chain_id` LE | `raw`.
pub fn body(chain_id: u64, raw: &[u8]) -> Vec<u8> {
    [&chain_id.to_le_bytes()[..], raw].concat()
}

/// The hash settle answered, or None for any other answer.
pub fn sent(r: Result<Vec<u8>, i32>) -> Option<[u8; 32]> {
    r.ok().and_then(|b| <[u8; 32]>::try_from(b.as_slice()).ok())
}
