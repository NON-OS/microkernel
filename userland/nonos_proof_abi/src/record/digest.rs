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

//! The two names a record goes by: ctx, the four words the device proof is
//! bound to, and record_id, the 32 bytes the anchor and a delivery cite.

use alloc::vec::Vec;

use super::layout::RECORD_BYTES;
use crate::container::Kind;

const CTX_DOMAIN: &[u8] = b"NONOS-PROOF-RECORD-v1";
const ID_CONTEXT: &str = "NONOS-PROOF-RECORD-ID-v1";

/// BLAKE3(domain | kind u16 | LE32 240 | record), read as the attestation
/// gates read a digest: four little-endian words, each reduced below p. The
/// kind is inside, so one record never stands for two kinds of proof.
pub fn ctx(kind: Kind, record: &[u8; RECORD_BYTES]) -> [u64; 4] {
    let mut b = Vec::with_capacity(CTX_DOMAIN.len() + 6 + RECORD_BYTES);
    b.extend_from_slice(CTX_DOMAIN);
    b.extend_from_slice(&(kind as u16).to_le_bytes());
    b.extend_from_slice(&(RECORD_BYTES as u32).to_le_bytes());
    b.extend_from_slice(record);
    nox_verify::attest::digest_words(blake3::hash(&b).as_bytes())
}

pub fn record_id(record: &[u8; RECORD_BYTES]) -> [u8; 32] {
    blake3::derive_key(ID_CONTEXT, record)
}
