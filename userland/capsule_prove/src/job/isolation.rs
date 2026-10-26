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

//! The isolation proof: the witness the sealed run and the policy's set make,
//! proven blinded and certified, again with fresh entropy when the rank
//! certificate is not reached (at most three tries), and wiped after. The
//! witness and every try's entropy are made on the serving thread; the tries
//! run on the worker (worker.rs).

use alloc::vec::Vec;

use isolation_stark::{prove as stark, Error, Statement, Witness};
use nonos_proof_abi::wire::PROVER;

use crate::assemble::wipe_bytes;
use crate::market::{witness, Sealed};

use super::isolation_wipe::clear;

const TRIES: usize = 3;

/// What the worker proves: the witness, its statement, a draw per try; the
/// witness and the draws wiped when it drops, proven or not.
pub struct Isolation {
    w: Witness,
    pub st: Statement,
    entropy: [[u8; 64]; TRIES],
}

impl Drop for Isolation {
    fn drop(&mut self) {
        clear(&mut self.w);
        self.entropy.iter_mut().for_each(|e| wipe_bytes(e));
    }
}

pub fn inputs(s: &Sealed, policy: &[u8; 256]) -> Result<Isolation, i32> {
    let (w, st) = witness(s, policy)?;
    let mut i = Isolation { w, st, entropy: [[0; 64]; TRIES] };
    for e in i.entropy.iter_mut() {
        *e = super::entropy::drawn().ok_or(PROVER)?;
    }
    Ok(i)
}

/// Worker: the proof.
pub fn prove(i: Isolation) -> Result<Vec<u8>, i32> {
    tries(&i.w, &i.st, &i.entropy)
}

fn tries(w: &Witness, st: &Statement, entropy: &[[u8; 64]; TRIES]) -> Result<Vec<u8>, i32> {
    for e in entropy {
        match stark(w, e) {
            Ok((got, proof)) if got == *st => return Ok(proof),
            Err(Error::Rank(_)) => continue,
            _ => return Err(PROVER),
        }
    }
    Err(PROVER)
}
