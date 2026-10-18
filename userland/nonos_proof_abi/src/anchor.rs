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
//!   ANCHOR_TAG = keccak256("NONOS.proof.anchor.v1")
//!   anchor = keccak256(abi.encode(ANCHOR_TAG, uint8 kind, bytes32 listingId,
//!                                 bytes32 record_id, bytes32 tag, bytes32 keccak256(container)))
//!
//! with the device tag's four words as 32 little-endian bytes. The listing is
//! inside, so a copy under another listing is another anchor; keccak, so a
//! contract recomputes it from calldata.

use nonos_stark::hash::keccak256;

use crate::container::{parse, tag, Kind};
use crate::device::decode_device;
use crate::record::record_id;
use crate::words::words_le;

pub const ANCHOR_LABEL: &[u8] = b"NONOS.proof.anchor.v1";

pub fn anchor_tag() -> [u8; 32] {
    keccak256(ANCHOR_LABEL)
}

pub fn anchor(
    kind: Kind,
    listing_id: &[u8; 32],
    record_id: &[u8; 32],
    tag: &[u64; 4],
    container: &[u8],
) -> [u8; 32] {
    let mut kind_word = [0u8; 32];
    kind_word[31] = kind.code();
    let words =
        [anchor_tag(), kind_word, *listing_id, *record_id, words_le(tag), keccak256(container)];
    keccak256(&words.concat())
}

/// The anchor of a kind-1 container under `listing_id`, from its own record
/// and device tag: `None` for bytes that are not one, so fetch never carries
/// anything else and a reader never keeps anything else.
pub fn anchor_of(listing_id: &[u8; 32], container: &[u8]) -> Option<[u8; 32]> {
    let c = parse(container)?;
    if c.kind != Kind::Isolation {
        return None;
    }
    let record = c.get(tag::RECORD)?.try_into().ok()?;
    let (st, _) = decode_device(c.get(tag::DEVICE)?)?;
    let device_tag = st.tag.map(|f| f.to_u64());
    Some(anchor(Kind::Isolation, listing_id, &record_id(record), &device_tag, container))
}
