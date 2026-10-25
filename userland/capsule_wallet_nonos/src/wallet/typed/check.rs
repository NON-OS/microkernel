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

/*
 * A typed signature is believed only for what the wallet asked (plan/31):
 * the signer is this account; the structHash is keccak256(typehash | words)
 * recomputed here; the domainSeparator is "NONOS Bond", version 1, this
 * chain and the config's bond, recomputed here; the digest is keccak256(0x1901
 * | domainSeparator | structHash); and v is 27 or 28. A reply signed under
 * any other domain is refused even if the keyring were wrong.
 */

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::{domain_words_at, TypedData, BOND};

use super::reply::parse_reply;

/// keccak256(typehash | words), as EIP-712 hashes a struct of static words.
pub fn struct_hash(k: &impl Keccak, t: &TypedData, words: &[[u8; 32]]) -> Option<[u8; 32]> {
    if words.len() != t.fields {
        return None;
    }
    let mut pre = alloc::vec::Vec::with_capacity(32 * (1 + words.len()));
    pre.extend_from_slice(&t.typehash);
    words.iter().for_each(|w| pre.extend_from_slice(w));
    Some(k.keccak256(&pre))
}

/// What the wallet asked op 30 for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Asked {
    pub signer: [u8; 20],
    pub struct_hash: [u8; 32],
    pub chain_id: u64,
    pub bond: [u8; 20],
}

/// keccak256(0x1901 | separator | struct_hash).
pub fn digest(k: &impl Keccak, separator: &[u8; 32], struct_hash: &[u8; 32]) -> [u8; 32] {
    let mut m = [0u8; 66];
    m[..2].copy_from_slice(&[0x19, 0x01]);
    m[2..34].copy_from_slice(separator);
    m[34..].copy_from_slice(struct_hash);
    k.keccak256(&m)
}

/// The signature in `reply`, when it is exactly what was asked.
pub fn believed(k: &impl Keccak, reply: &[u8], a: &Asked) -> Option<[u8; 65]> {
    let s = parse_reply(reply)?;
    let separator = k.keccak256(&domain_words_at(&BOND, a.chain_id, a.bond));
    let ok = s.signer == a.signer
        && s.struct_hash == a.struct_hash
        && s.domain_separator == separator
        && s.digest == digest(k, &separator, &s.struct_hash)
        && matches!(s.sig[64], 27 | 28);
    ok.then_some(s.sig)
}
