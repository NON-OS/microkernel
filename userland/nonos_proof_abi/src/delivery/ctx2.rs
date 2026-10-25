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

//! ctx2, the four words a delivery's device proof is bound to:
//! digest_words(BLAKE3("NONOS-PROOF-DELIVERY-v1" | LE32 240 | record | LE32
//! 208 | body)). Both lengths are inside, and the domain is not a record's
//! ctx's, so a delivery's proof never stands for an isolation proof.

use crate::delivery::DELIVERY_BYTES;
use crate::record::RECORD_BYTES;

const CTX2_DOMAIN: &[u8] = b"NONOS-PROOF-DELIVERY-v1";

pub fn ctx2(record: &[u8; RECORD_BYTES], body: &[u8; DELIVERY_BYTES]) -> [u64; 4] {
    let mut h = blake3::Hasher::new();
    h.update(CTX2_DOMAIN);
    h.update(&(RECORD_BYTES as u32).to_le_bytes());
    h.update(record);
    h.update(&(DELIVERY_BYTES as u32).to_le_bytes());
    h.update(body);
    nox_verify::attest::digest_words(h.finalize().as_bytes())
}
