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

//! One LiveGate per bucket: live is a_b, the value's low limb is ok_b. A
//! clear bit forces both limbs to zero, so nothing outside A succeeded. A set
//! bit needs a nonzero value, so a bucket in A that never succeeded carries 1
//! in the free high limb. The gate's other cells bind nothing here and stay
//! zero.

use alloc::vec::Vec;

use nonos_proof_abi::{allows, BUCKETS};
use nonos_stark::air::{LiveGate, LANES};
use nonos_stark::field::Fp;

use crate::statement::Witness;

fn gate(live: bool, ok: u64) -> LiveGate {
    let hi = u64::from(live && ok == 0);
    let value = Fp::from_u64(ok) + Fp::from_u64(hi) * Fp::from_u64(1u64 << 32);
    let inv = if value == Fp::ZERO { 0 } else { value.inv().to_u64() };
    let z = [0; LANES];
    LiveGate {
        log_t: 1,
        live,
        value: [ok, hi],
        inv,
        walked_note: z,
        note_root: z,
        walked_assoc: z,
        assoc_root: z,
        dead: 0,
        dead_domain: 0,
    }
}

/// The gates a witness fills; all dead and zero for the verifier's shape.
pub(super) fn gates(w: Option<&Witness>) -> Vec<LiveGate> {
    (0..BUCKETS)
        .map(|b| match w {
            Some(w) => gate(allows(&w.set, b), u64::from(w.hist[2 * b])),
            None => gate(false, 0),
        })
        .collect()
}
