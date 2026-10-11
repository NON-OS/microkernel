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

//! Steps 3 to 5: the measurements, the app and the public words all agree
//! with the record and the policy.

use alloc::vec::Vec;

use nonos_stark::field::P;

use super::expected::PolicyView;
use super::refusal::Refusal;
use crate::meas::meas_chain;
use crate::record::Record;
use crate::syscall_root::syscall_root;
use crate::words::u64_at;

const POLICY_DOMAIN: &[u8] = b"NONOS.linux.policy.v1";
const APP_DOMAIN: &[u8] = b"NONOS.linux.app.v1";

fn hash(parts: &[&[u8]]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    parts.iter().for_each(|p| {
        h.update(p);
    });
    *h.finalize().as_bytes()
}

/// app_id = BLAKE3("NONOS.linux.app.v1" | program_blake3 | policy_hash),
/// policy_hash = BLAKE3("NONOS.linux.policy.v1" | the policy's bytes).
pub fn app_id(policy: &[u8], program_blake3: &[u8; 32]) -> [u8; 32] {
    let policy_hash = hash(&[POLICY_DOMAIN, policy]);
    hash(&[APP_DOMAIN, program_blake3, &policy_hash])
}

/// The list rebuilds M exactly, link for link, and names the program.
pub(super) fn measurements(list: &[u8], rec: &Record, program: &[u8; 32]) -> Result<(), Refusal> {
    let entries: Vec<[u8; 32]> = list.chunks_exact(32).filter_map(|c| c.try_into().ok()).collect();
    let whole = entries.len() as u64 == u64::from(rec.n_meas);
    if !whole || meas_chain(&entries) != rec.meas || !entries.contains(program) {
        return Err(Refusal::Measurements);
    }
    Ok(())
}

/// The ten words, canonical, with Hc the record's and A the policy's set.
pub(super) fn publics(b: &[u8], rec: &Record, view: &PolicyView) -> Result<[u64; 8], Refusal> {
    let mut w = [0u64; 8];
    for (i, v) in w.iter_mut().enumerate() {
        *v = u64_at(b, 8 * i).ok_or(Refusal::Format)?;
    }
    let a = syscall_root(&view.syscall_set).ok_or(Refusal::Policy)?;
    if w.iter().any(|&v| v >= P) || w[..4] != rec.hc || w[4..8] != a {
        return Err(Refusal::Publics);
    }
    Ok(w)
}
