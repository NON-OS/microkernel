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
 * the keyring's signer must be this account, and its structHash must equal
 * keccak256(typehash | words) recomputed here from the config's typehash.
 * Anything else is a keyring that signed something else, and is refused.
 */

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::TypedData;

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

/// The signature in `reply`, when it is `signer`'s over `expected`.
pub fn believed(reply: &[u8], signer: &[u8; 20], expected: &[u8; 32]) -> Option<[u8; 65]> {
    let s = parse_reply(reply)?;
    (s.signer == *signer && s.struct_hash == *expected).then_some(s.sig)
}
