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

//! The isolation half of a market proof: the witness the sealed run and the
//! policy's set make, held to the record before anything is proven.

use isolation_stark::{Statement, Witness};
use nonos_proof_abi::wire::{OUTSIDE, PROVER};
use nonos_proof_abi::{allows, syscall_root, BUCKETS};

use super::sealed::Sealed;

/// plan/30: the syscall_set, 65 bytes at offset 152.
pub fn set(policy: &[u8; 256]) -> [u8; 65] {
    let mut s = [0u8; 65];
    s.copy_from_slice(&policy[152..217]);
    s
}

/// The witness and the statement it proves: OUTSIDE when a call outside the
/// set succeeded, PROVER when its Hc is not the record's, which would mean
/// the kernel and this prover disagree about H.
pub fn witness(s: &Sealed, policy: &[u8; 256]) -> Result<(Witness, Statement), i32> {
    let set = set(policy);
    let w = Witness { hist: s.hist, salt: s.salt, set };
    let outside = (0..BUCKETS).any(|b| w.hist[2 * b] > 0 && !allows(&set, b));
    let st = Statement::of(&w).ok_or(if outside { OUTSIDE } else { PROVER })?;
    if st.hc != s.record.hc || Some(st.a) != syscall_root(&set) {
        return Err(PROVER);
    }
    Ok((w, st))
}
