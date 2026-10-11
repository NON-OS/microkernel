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

//! One listing as a document of its own: the bytes of
//! one schema 3 entry exactly as an index carries them, nothing after. The
//! wallet's publisher screen reads it to sign PublisherBinding (entry 2) and
//! each release's ReleaseAttest (entry 3); marketplace-index writes it.

use super::decode_entry;
use super::error::DecodeError;
use super::reader::Reader;
use super::schema::SCHEMA_V3;
use crate::types::MarketplaceEntry;

pub fn decode_entry_document(buf: &[u8]) -> Result<MarketplaceEntry, DecodeError> {
    let mut r = Reader::new(buf);
    let entry = decode_entry::read(&mut r, SCHEMA_V3)?;
    if r.remaining() != 0 {
        return Err(DecodeError::Trailing);
    }
    Ok(entry)
}

/// The document marketplace-index writes for `entry`.
#[cfg(feature = "canonical-encode")]
pub fn encode_entry_document(entry: &MarketplaceEntry) -> alloc::vec::Vec<u8> {
    let mut out = alloc::vec::Vec::new();
    super::encode_entry::write(&mut super::writer::Writer::new(&mut out), entry);
    out
}
