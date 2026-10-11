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

//! What a contract's ERC-5267 eip712Domain() must return for `Domain` on a
//! chain: fields 0x0f (name, version, chainId, verifyingContract), the name
//! and version strings, the chain id, the contract, a zero salt and no
//! extensions, as standard ABI. Anything else is another contract.

use super::consts::Domain;

fn word(ret: &[u8], i: usize) -> Option<&[u8]> {
    ret.get(i.checked_mul(32)?..i.checked_mul(32)?.checked_add(32)?)
}

fn small(w: &[u8]) -> Option<usize> {
    w[..24]
        .iter()
        .all(|b| *b == 0)
        .then(|| u64::from_be_bytes(w[24..].try_into().unwrap()) as usize)
}

/* The string at the head word `i` points to, and the offset it ends at. */
fn string(ret: &[u8], i: usize) -> Option<(&[u8], usize)> {
    let at = small(word(ret, i)?)?;
    let len = small(ret.get(at..at.checked_add(32)?)?)?;
    let start = at + 32;
    let text = ret.get(start..start.checked_add(len)?)?;
    let padded = start + len.div_ceil(32) * 32;
    ret.get(start + len..padded)?.iter().all(|b| *b == 0).then_some((text, padded))
}

/// Whether `ret`, the raw return of eip712Domain(), is exactly `d` on
/// `chain_id` at `contract`.
pub fn domain_answer_matches(d: &Domain, chain_id: u64, contract: &[u8; 20], ret: &[u8]) -> bool {
    let check = || -> Option<bool> {
        let fields = word(ret, 0)?;
        let mut id = [0u8; 32];
        id[24..].copy_from_slice(&chain_id.to_be_bytes());
        let mut at = [0u8; 32];
        at[12..].copy_from_slice(contract);
        let heads = fields[0] == 0x0f
            && fields[1..].iter().all(|b| *b == 0)
            && word(ret, 3)? == id
            && word(ret, 4)? == at
            && word(ret, 5)?.iter().all(|b| *b == 0);
        let (name, end_name) = string(ret, 1)?;
        let (version, end_version) = string(ret, 2)?;
        let ext = small(word(ret, 6)?)?;
        let empty = small(ret.get(ext..ext.checked_add(32)?)?)? == 0;
        let end = end_name.max(end_version).max(ext + 32);
        Some(
            heads
                && name == d.name.as_bytes()
                && version == d.version.as_bytes()
                && empty
                && end == ret.len(),
        )
    };
    check().unwrap_or(false)
}
