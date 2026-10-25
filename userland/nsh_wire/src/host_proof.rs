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

//! `HOST_PROOF`: the host keys, the enrolment certificate, the kernel's
//! attestation document and the registry entries, signed by the host keys.
//! The document is placed here and read by the attestation checker; the
//! entries are only held to their 45-byte record size.

use crate::keys::{before_sig, HybridPublic, HybridSig};
use crate::{Enrol, Reader, WireError};

/// The largest `nsh-enrol/1`: the fixed fields, a full host name and
/// every boot application.
pub const MAX_ENROL: usize =
    4 + 2 + 2 + 16 + 1 + 253 + 64 + 64 + 4 * 32 + 2 + 8 * 32 + 8 + 32 + 64 + 3309;
/// The largest attestation document `HOST_PROOF` may carry.
pub const MAX_DOC: usize = 196_608;
/// One registry entry (`src/security/attest_registry/format.rs`).
pub const ENTRY_LEN: usize = 45;
pub const MAX_ENTRIES: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostProof<'a> {
    pub host: HybridPublic<'a>,
    pub enrol: Enrol<'a>,
    pub doc: &'a [u8],
    pub entries: &'a [u8],
    pub sig: HybridSig<'a>,
    /// The bytes the transcript for the host signature covers.
    pub signed: &'a [u8],
}

impl<'a> HostProof<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(body);
        let host = HybridPublic::read(&mut r)?;
        let enrol = Enrol::parse(r.bytes16(1, MAX_ENROL)?)?;
        let doc = r.bytes32(1, MAX_DOC)?;
        let entries = r.bytes32(ENTRY_LEN, ENTRY_LEN * MAX_ENTRIES)?;
        if entries.len() % ENTRY_LEN != 0 {
            return Err(WireError::Value);
        }
        let sig = HybridSig::read(&mut r)?;
        r.finish()?;
        Ok(HostProof { host, enrol, doc, entries, sig, signed: before_sig(body)? })
    }

    /// The registry entries one by one.
    pub fn entries(&self) -> impl Iterator<Item = &'a [u8; ENTRY_LEN]> + 'a {
        self.entries.chunks_exact(ENTRY_LEN).filter_map(|e| e.try_into().ok())
    }
}
