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

//! A verifier's request, written field by field as the capsule's request
//! format documents it.

use nonos_device_attest::Registry;

pub const VERIFIER: &[u8] = b"faucet.nonos.software";
pub const WINDOW: u64 = 20_000;
pub const NONCE: [u8; 32] = [0x42; 32];

pub fn request(verifier: &[u8], window: u64, nonce: &[u8; 32], root: &[u8; 32]) -> Vec<u8> {
    let mut r = b"NZKDREQ1".to_vec();
    r.extend(window.to_le_bytes());
    r.extend(nonce);
    r.extend(root);
    r.push(verifier.len() as u8);
    r.extend(verifier);
    r
}

/// A registry's root as the request carries it: four words, little-endian.
pub fn root_bytes(r: &Registry) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (o, w) in out.chunks_exact_mut(8).zip(r.root()) {
        o.copy_from_slice(&w.to_u64().to_le_bytes());
    }
    out
}
