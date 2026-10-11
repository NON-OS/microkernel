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

//! The five words the domain separator hashes. This crate holds no keccak:
//! the keyring and the capsules hash with the kernel's, the host tools with
//! their own, and all of them hash these same 160 bytes.

use super::consts::{Domain, EIP712_DOMAIN_TYPEHASH, VERSION_HASH};
use crate::error::ConfigError;
use crate::lookup::address;

/// `abi.encode(typehash, keccak(name), keccak(version), chainId, contract)`
/// for an explicit contract address.
pub const fn domain_words_at(d: &Domain, chain_id: u64, contract: [u8; 20]) -> [u8; 160] {
    let mut out = [0u8; 160];
    let mut i = 0;
    while i < 32 {
        out[i] = EIP712_DOMAIN_TYPEHASH[i];
        out[32 + i] = d.name_hash[i];
        out[64 + i] = VERSION_HASH[i];
        i += 1;
    }
    let id = chain_id.to_be_bytes();
    let mut j = 0;
    while j < 8 {
        out[96 + 24 + j] = id[j];
        j += 1;
    }
    let mut k = 0;
    while k < 20 {
        out[128 + 12 + k] = contract[k];
        k += 1;
    }
    out
}

/// The same, with the contract the config holds for this chain. An unknown
/// chain or an undeployed contract refuses, so nothing signs for address zero.
pub const fn domain_words(d: &Domain, chain_id: u64) -> Result<[u8; 160], ConfigError> {
    match address(chain_id, d.contract) {
        Ok(a) => Ok(domain_words_at(d, chain_id, a)),
        Err(e) => Err(e),
    }
}
