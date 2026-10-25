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

//! One `TCG_PCR_EVENT2`, and the extend a TPM makes for it.

use nonos_hash::{sha256, Sha256};

use super::tcg::{SHA1, SHA256};

pub fn event(pcr: u32, kind: u32, digests: &[(u16, Vec<u8>)], body: &[u8]) -> Vec<u8> {
    let mut e = pcr.to_le_bytes().to_vec();
    e.extend(kind.to_le_bytes());
    e.extend((digests.len() as u32).to_le_bytes());
    for (alg, d) in digests {
        e.extend(alg.to_le_bytes());
        e.extend(d);
    }
    e.extend((body.len() as u32).to_le_bytes());
    e.extend(body);
    e
}

/// An event in a SHA-1 and SHA-256 log, as a PC logs most of them.
pub fn ev(pcr: u32, kind: u32, d: [u8; 32]) -> Vec<u8> {
    event(pcr, kind, &[(SHA1, vec![0x5A; 20]), (SHA256, d.to_vec())], b"event data")
}

pub fn extend(pcr: &[u8; 32], d: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(pcr);
    h.update(d);
    h.finalize()
}

pub fn digest(tag: &[u8]) -> [u8; 32] {
    sha256(tag)
}

/// A separator or action: its SHA-256 is its body's, as PCR 4 requires.
pub fn note_ev(pcr: u32, kind: u32, body: &[u8]) -> Vec<u8> {
    event(pcr, kind, &[(SHA1, vec![0x5A; 20]), (SHA256, sha256(body).to_vec())], body)
}
