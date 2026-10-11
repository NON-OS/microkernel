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

//! The bytes keccak256 is taken over. `keccak` is the device's syscall in
//! the keyring and a library on the host, so this file is the same in both.

use alloc::vec::Vec;

use super::domain::{Domain, DOMAIN_TYPE};
use super::entry::Entry;

/// typehash | keccak(name) | keccak(version) | chainId | verifyingContract.
pub fn domain_preimage(
    d: &Domain,
    chain: u64,
    contract: &[u8; 20],
    keccak: impl Fn(&[u8]) -> Option<[u8; 32]>,
) -> Option<[u8; 160]> {
    let mut out = [0u8; 160];
    out[0..32].copy_from_slice(&keccak(DOMAIN_TYPE.as_bytes())?);
    out[32..64].copy_from_slice(&keccak(d.name.as_bytes())?);
    out[64..96].copy_from_slice(&keccak(d.version.as_bytes())?);
    out[120..128].copy_from_slice(&chain.to_be_bytes());
    out[140..160].copy_from_slice(contract);
    Some(out)
}

/// typehash | the field words, which admit() has already checked. Every
/// field is a static type, so encodeData is the words as they came.
pub fn struct_preimage(
    e: &Entry,
    words: &[u8],
    keccak: impl Fn(&[u8]) -> Option<[u8; 32]>,
) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(32 + words.len());
    out.extend_from_slice(&keccak(e.type_string.as_bytes())?);
    out.extend_from_slice(words);
    Some(out)
}

/// 0x19 0x01 | domainSeparator | structHash.
pub fn digest_preimage(domain_separator: &[u8; 32], struct_hash: &[u8; 32]) -> [u8; 66] {
    let mut out = [0u8; 66];
    out[0] = 0x19;
    out[1] = 0x01;
    out[2..34].copy_from_slice(domain_separator);
    out[34..66].copy_from_slice(struct_hash);
    out
}
