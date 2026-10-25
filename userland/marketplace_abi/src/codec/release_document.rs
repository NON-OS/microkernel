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

//! One release as a document of its own: what a publisher serves at the uri
//! its NoxListingRegistry Published event names, the bytes of one
//! schema 3 release exactly as an index carries them, nothing after. A
//! reader recomputes the signing bytes from the decoded release, so the
//! release hash it checks is the v3 signing form's, never the raw bytes'.

use super::decode_release;
use super::error::DecodeError;
use super::reader::Reader;
use super::schema::SCHEMA_V3;
use crate::types::CapsuleRelease;

pub fn decode_release_document(buf: &[u8]) -> Result<CapsuleRelease, DecodeError> {
    let mut r = Reader::new(buf);
    let release = decode_release::read(&mut r, SCHEMA_V3)?;
    if r.remaining() != 0 {
        return Err(DecodeError::Trailing);
    }
    Ok(release)
}

/// The document a publisher serves for a schema 3 release; None for one
/// without `ext`, which has no v3 signing form.
#[cfg(feature = "canonical-encode")]
pub fn encode_release_document(release: &CapsuleRelease) -> Option<alloc::vec::Vec<u8>> {
    release.ext.as_ref()?;
    let mut out = alloc::vec::Vec::new();
    super::encode_release::write(&mut super::writer::Writer::new(&mut out), release);
    Some(out)
}
