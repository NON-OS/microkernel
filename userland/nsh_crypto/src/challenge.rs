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

//! The attestation challenge, and the digest that names a hybrid key pair.

use nonos_hash::{Sha256, Sha512};

/// The 32 bytes `HOST_PROOF`'s attestation document must carry as its
/// challenge: `SHA-256("nsh/1 attest\0" || TH(SERVER_HELLO) || H(host keys))`.
pub fn attest_challenge(th_server_hello: &[u8; 64], host_ed: &[u8], host_ml: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"nsh/1 attest\0");
    h.update(th_server_hello);
    h.update(&key_digest(host_ed, host_ml));
    h.finalize()
}

/// `H(host keys)` or a subject: SHA-512 of the Ed25519 key, then the
/// ML-DSA-65 key.
pub fn key_digest(ed: &[u8], ml: &[u8]) -> [u8; 64] {
    let mut h = Sha512::new();
    h.update(ed);
    h.update(ml);
    h.finalize()
}
