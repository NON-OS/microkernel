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
//! certificate is not reached (at most three tries), and wiped after.

use alloc::vec::Vec;

use isolation_stark::{prove, Error, Statement, Witness};
use nonos_proof_abi::wire::PROVER;

use crate::assemble::wipe_bytes;
use crate::market::{witness, Sealed};

const TRIES: usize = 3;

pub fn proof(s: &Sealed, policy: &[u8; 256]) -> Result<(Vec<u8>, Statement), i32> {
    let (mut w, st) = witness(s, policy)?;
    let out = tries(&w, &st);
    clear(&mut w);
    out.map(|p| (p, st))
}

fn tries(w: &Witness, st: &Statement) -> Result<Vec<u8>, i32> {
    for _ in 0..TRIES {
        let mut entropy = [0u8; 64];
        if nonos_libc::crypto_random(entropy.as_mut_ptr(), entropy.len()) < 0 {
            return Err(PROVER);
        }
        let made = prove(w, &entropy);
        wipe_bytes(&mut entropy);
        match made {
            Ok((got, proof)) if got == *st => return Ok(proof),
            Err(Error::Rank(_)) => continue,
            _ => return Err(PROVER),
        }
    }
    Err(PROVER)
}

pub(super) fn clear(w: &mut Witness) {
    // SAFETY: valid, aligned fields of `w`, borrowed mutably.
    w.hist.iter_mut().for_each(|h| unsafe { core::ptr::write_volatile(h, 0) });
    w.salt.iter_mut().for_each(|x| unsafe { core::ptr::write_volatile(x, 0) });
}
