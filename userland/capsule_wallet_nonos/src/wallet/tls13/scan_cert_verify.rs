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

//! One record's handshake messages, read for the leaf and its signature.

use alloc::vec::Vec;

const CERTIFICATE: u8 = 11;
const CERTIFICATE_VERIFY: u8 = 15;

/// Some(verdict) once CertificateVerify is reached; None to keep reading.
pub fn scan(transcript: &mut Vec<u8>, leaf: &mut Option<Vec<u8>>, msgs: &[u8]) -> Option<bool> {
    let mut pos = 0usize;
    while pos + 4 <= msgs.len() {
        let len = ((msgs[pos + 1] as usize) << 16)
            | ((msgs[pos + 2] as usize) << 8)
            | msgs[pos + 3] as usize;
        let end = pos + 4 + len;
        if end > msgs.len() {
            return Some(false);
        }
        let body = &msgs[pos + 4..end];
        match msgs[pos] {
            CERTIFICATE => *leaf = super::cert_at::cert_at(body, 0).map(<[u8]>::to_vec),
            CERTIFICATE_VERIFY => {
                let Some(cert) = leaf.as_deref() else { return Some(false) };
                return Some(super::cert_verify::check(cert, transcript, body));
            }
            _ => {}
        }
        transcript.extend_from_slice(&msgs[pos..end]);
        pos = end;
    }
    None
}
