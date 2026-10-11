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

//! The 32 bytes that go on chain for a proof.
//!
//! anchor = keccak256(abi.encodePacked("NONOS-PROOF-ANCHOR-v1", uint8 kind,
//! bytes32 record_id, bytes32 tag, bytes32 keccak256(container))), with the
//! device tag's four words as 32 little-endian bytes. Keccak so a contract
//! can recompute it from calldata.

use alloc::vec::Vec;

use nonos_stark::hash::keccak256;

use crate::container::Kind;
use crate::words::words_le;

pub const ANCHOR_DOMAIN: &[u8] = b"NONOS-PROOF-ANCHOR-v1";

pub fn anchor(kind: Kind, record_id: &[u8; 32], tag: &[u64; 4], container: &[u8]) -> [u8; 32] {
    let mut b = Vec::with_capacity(ANCHOR_DOMAIN.len() + 1 + 96);
    b.extend_from_slice(ANCHOR_DOMAIN);
    b.push(kind.code());
    b.extend_from_slice(record_id);
    b.extend_from_slice(&words_le(tag));
    b.extend_from_slice(&keccak256(container));
    keccak256(&b)
}
