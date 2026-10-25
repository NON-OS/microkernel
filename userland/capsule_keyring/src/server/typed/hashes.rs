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

//! The three hashes of an EIP-712 signature.

use nonos_nox_config::domain_words_at;

use super::entry::Entry;
use super::preimage::{digest_preimage, struct_preimage};

pub struct Hashes {
    pub domain_separator: [u8; 32],
    pub struct_hash: [u8; 32],
    pub digest: [u8; 32],
}

pub fn hashes(
    e: &Entry,
    chain: u64,
    contract: &[u8; 20],
    words: &[u8],
    keccak: impl Fn(&[u8]) -> Option<[u8; 32]>,
) -> Option<Hashes> {
    let domain_separator = keccak(&domain_words_at(e.typed.domain, chain, *contract))?;
    let struct_hash = keccak(&struct_preimage(e, words))?;
    let digest = keccak(&digest_preimage(&domain_separator, &struct_hash))?;
    Some(Hashes { domain_separator, struct_hash, digest })
}
