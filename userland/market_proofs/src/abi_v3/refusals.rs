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

//! Every byte schema 3 adds is read strictly: an unknown kind is refused,
//! never read as free, and nothing rides after the signature.

use nonos_marketplace_abi::{commerce_bytes, DecodeError};

use super::blobs::{blob, decode, find};
use super::fixtures::sold;

// payee, bond, unit, price, minimum: the settle mode follows them.
pub(super) const SETTLE_AT: usize = 20 + 32 + 32 + 16 + 16;

fn commerce_at(b: &[u8]) -> usize {
    find(b, &commerce_bytes(sold().entries[0].commerce.as_ref().unwrap()))
}

#[test]
fn a_byte_after_the_signature_is_refused() {
    let mut b = blob(sold());
    b.push(0);
    assert_eq!(decode(&b).err(), Some(DecodeError::Trailing));
}

#[test]
fn an_unknown_price_kind_is_not_free() {
    let mut b = blob(sold());
    // The price follows the description and, from schema 4, the logo hash.
    let at = find(&b, b"DESC") + 4 + 32;
    b[at] = 9;
    assert_eq!(decode(&b).err(), Some(DecodeError::UnknownKind));
}

#[test]
fn an_unknown_settle_mode_or_proof_kind_is_refused() {
    for (offset, value) in [(SETTLE_AT, 3u8), (SETTLE_AT + 1 + 4, 0), (SETTLE_AT + 1 + 4, 4)] {
        let mut b = blob(sold());
        let at = commerce_at(&b) + offset;
        b[at] = value;
        assert_eq!(decode(&b).err(), Some(DecodeError::UnknownKind), "offset {offset}");
    }
}

#[test]
fn a_guest_policy_must_be_256_bytes_or_none() {
    let mut b = blob(sold());
    let mut prefix = 256u32.to_le_bytes().to_vec();
    prefix.extend_from_slice(&[0x9a; 8]);
    let at = find(&b, &prefix);
    b[at..at + 4].copy_from_slice(&255u32.to_le_bytes());
    assert_eq!(decode(&b).err(), Some(DecodeError::BadLength));
}
