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

//! Crafted TCG logs in the crypto-agile format, built field by field the way
//! the TCG PC Client specification lays them out.

use sha2::{Digest, Sha256};

pub(super) const SHA1: u16 = 0x0004;
pub(super) const SHA256: u16 = 0x000B;
pub(super) const SHA384: u16 = 0x000C;
pub(super) const EV_SEPARATOR: u32 = 0x0000_0004;
pub(super) const EV_EFI_ACTION: u32 = 0x8000_0007;
pub(super) const EV_APP: u32 = 0x8000_0003;

pub(super) fn header(banks: &[(u16, u16)]) -> Vec<u8> {
    let mut body = b"Spec ID Event03\0".to_vec();
    body.extend(0u32.to_le_bytes());
    body.extend([0, 2, 0, 2]);
    body.extend((banks.len() as u32).to_le_bytes());
    for (a, s) in banks {
        body.extend(a.to_le_bytes());
        body.extend(s.to_le_bytes());
    }
    body.push(0);
    let mut h = 0u32.to_le_bytes().to_vec();
    h.extend(3u32.to_le_bytes());
    h.extend([0u8; 20]);
    h.extend((body.len() as u32).to_le_bytes());
    h.extend(body);
    h
}

pub(super) fn event(pcr: u32, kind: u32, digests: &[(u16, Vec<u8>)], body: &[u8]) -> Vec<u8> {
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
pub(super) fn ev(pcr: u32, kind: u32, sha256: [u8; 32]) -> Vec<u8> {
    event(pcr, kind, &[(SHA1, vec![0x5A; 20]), (SHA256, sha256.to_vec())], b"event data")
}

pub(super) fn standard() -> Vec<u8> {
    header(&[(SHA1, 20), (SHA256, 32)])
}

pub(super) fn extend(pcr: [u8; 32], d: [u8; 32]) -> [u8; 32] {
    Sha256::new().chain_update(pcr).chain_update(d).finalize().into()
}

pub(super) fn digest(tag: u8) -> [u8; 32] {
    Sha256::digest([tag]).into()
}
