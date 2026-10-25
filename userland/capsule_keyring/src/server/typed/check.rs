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

//! What must hold before, and after, the key is touched.

use nonos_nox_config::{deployment, Chain, Deployment};

use super::entry::Entry;
use super::field::{well_formed, Kind};
use super::refusal::Refusal;

/// Whether `sender_pid` owns one of the entry's services, given `owner_of`,
/// the pid that owns a name. A dead or unknown name owns nothing.
pub fn caller_allowed(
    entry: &Entry,
    sender_pid: u32,
    owner_of: impl Fn(&[u8]) -> Option<u32>,
) -> bool {
    sender_pid != 0 && entry.callers.iter().any(|name| owner_of(name) == Some(sender_pid))
}

/// Everything that can be decided without the key: the chain, the
/// contract, and the shape of every field. Returns the contract.
pub fn admit(entry: &Entry, chain: u64, words: &[u8]) -> Result<[u8; 20], Refusal> {
    admit_in(entry, chain, words, deployment)
}

/// admit() against `deployment` in place of the config's own, so the
/// proofs can show a deployed address reach the domain before one exists.
pub fn admit_in(
    entry: &Entry,
    chain: u64,
    words: &[u8],
    deployment: fn(Chain) -> &'static Deployment,
) -> Result<[u8; 20], Refusal> {
    let chain = Chain::from_id(chain).ok_or(Refusal::Chain)?;
    let contract = entry.contract(chain, deployment).ok_or(Refusal::NotDeployed)?;
    if words.len() != entry.fields.len() * 32 {
        return Err(Refusal::Malformed);
    }
    for (i, (f, w)) in entry.fields.iter().zip(words.chunks_exact(32)).enumerate() {
        let w: &[u8; 32] = w.try_into().map_err(|_| Refusal::Malformed)?;
        if !well_formed(f.kind, w) {
            return Err(Refusal::Field(i));
        }
    }
    Ok(contract)
}

/// Every field marked as the signer must be the signer's address.
pub fn signer_rule(entry: &Entry, words: &[u8], signer: &[u8; 20]) -> Result<(), Refusal> {
    for (f, w) in entry.fields.iter().zip(words.chunks_exact(32)) {
        if f.is_signer && (f.kind != Kind::Address || w[12..] != signer[..]) {
            return Err(Refusal::NotSigner);
        }
    }
    Ok(())
}
