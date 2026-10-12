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

//! The one place the station runs whichever cipher a key is for, so the
//! data path, the receive check and management frame protection cannot pick
//! differently. Both ciphers put the same 8-octet header after the MAC
//! header; only the MIC length and the block cipher differ.

use alloc::vec::Vec;

use crate::dot11::ccmp::{self, view};
use crate::dot11::gcmp;
use crate::dot11::header::FC_PROTECTED;
use crate::rsn::TemporalKey;

/// Protect a plaintext MPDU under `key` at packet number `pn`.
pub(super) fn seal(frame: &[u8], pn: u64, key: &TemporalKey) -> Option<Vec<u8>> {
    match key {
        TemporalKey::Ccmp128(k) => ccmp::encrypt(frame, pn, k),
        TemporalKey::Gcmp256(k) => gcmp::encrypt(frame, pn, k),
    }
}

/// The plaintext body of a protected MPDU, or `None` if it is malformed or
/// its MIC fails under `key`.
pub(super) fn open(frame: &[u8], key: &TemporalKey) -> Option<Vec<u8>> {
    match key {
        TemporalKey::Ccmp128(k) => ccmp::decrypt(frame, k),
        TemporalKey::Gcmp256(k) => gcmp::decrypt(frame, k),
    }
}

/// The MPDU with its protection removed: the MAC header with Protected
/// cleared, then the plaintext body.
pub(super) fn unprotect(frame: &[u8], key: &TemporalKey) -> Option<Vec<u8>> {
    let v = view(frame)?;
    let body = open(frame, key)?;
    let mut out = Vec::with_capacity(v.hdr_len + body.len());
    out.extend_from_slice(&frame[..v.hdr_len]);
    out[1] &= !((FC_PROTECTED >> 8) as u8);
    out.extend_from_slice(&body);
    Some(out)
}
