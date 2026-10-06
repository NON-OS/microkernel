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


//! The blinded key a service publishes under for one time period, and the
//! subcredential its descriptor and introduction are keyed with
//! (build_blinded_key_param and hs_get_subcredential in the fork).

use crate::crypto::keccak::sha3_256_parts;

/// "Derive temporary signing key" with its terminating NUL: the fork hashes
/// sizeof(blind_str), which counts it.
const BLIND_STRING: &[u8] = b"Derive temporary signing key\0";
const BASEPOINT: &[u8] = b"(15112221349535400772501151409588531511454012693041857206046113283949847762202, 46316835694926478169428394003475163141307993866256225615783033603165251855960)";

/// The blinded key for `identity` in period `period` of `length` minutes.
/// `None` when `identity` is not a point on the curve.
pub fn blinded_key(identity: &[u8; 32], period: u64, length: u64) -> Option<[u8; 32]> {
    let mut nonce = [0u8; 9 + 16];
    nonce[..9].copy_from_slice(b"key-blind");
    nonce[9..17].copy_from_slice(&period.to_be_bytes());
    nonce[17..].copy_from_slice(&length.to_be_bytes());
    let param = sha3_256_parts(&[BLIND_STRING, identity, BASEPOINT, &nonce]);
    nonos_ed25519::blind_public(identity, &param)
}

/// The subcredential for `identity` under `blinded`.
pub fn subcredential(identity: &[u8; 32], blinded: &[u8; 32]) -> [u8; 32] {
    let credential = sha3_256_parts(&[b"credential", identity]);
    sha3_256_parts(&[b"subcredential", &credential, blinded])
}
