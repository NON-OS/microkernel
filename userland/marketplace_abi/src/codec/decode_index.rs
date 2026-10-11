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

extern crate alloc;

use alloc::vec::Vec;

use super::decode_entry;
use super::error::DecodeError;
use super::reader::Reader;
use super::schema::readable;
use super::strings::{bounded_bytes, bounded_count, bounded_string};
use crate::limits::{MAX_ENTRIES, MAX_INDEX_BLOB, MAX_PUBLISHER, MAX_SIGNATURE};
use crate::types::{MarketplaceEntry, MarketplaceIndex};

pub struct DecodedIndex<'a> {
    pub index: MarketplaceIndex,
    pub signed_bytes: &'a [u8],
}

pub fn decode_index(buf: &[u8]) -> Result<DecodedIndex<'_>, DecodeError> {
    if buf.len() > MAX_INDEX_BLOB {
        return Err(DecodeError::BlobTooLarge);
    }
    let mut r = Reader::new(buf);

    let schema_version = r.u32()?;
    if !readable(schema_version) {
        return Err(DecodeError::UnsupportedSchema);
    }
    let operator_id = bounded_string(&mut r, MAX_PUBLISHER)?;
    let operator_pubkey = r.fixed::<32>()?;
    let published_at_ms = r.u64()?;
    let serial = r.u64()?;

    let entry_count = bounded_count(&mut r, MAX_ENTRIES)?;
    let mut entries: Vec<MarketplaceEntry> = Vec::with_capacity(entry_count as usize);
    for _ in 0..entry_count {
        entries.push(decode_entry::read(&mut r, schema_version)?);
    }

    let signed_end = r.position();
    let signed_bytes = r.slice_up_to(signed_end)?;
    let index_signature = bounded_bytes(&mut r, MAX_SIGNATURE)?;
    // Anything after the signature is covered by nothing, so it is refused
    // rather than carried along beside bytes that are.
    if r.remaining() != 0 {
        return Err(DecodeError::Trailing);
    }

    let index = MarketplaceIndex {
        schema_version,
        operator_id,
        operator_pubkey,
        published_at_ms,
        serial,
        entries,
        index_signature,
    };
    Ok(DecodedIndex { index, signed_bytes })
}
