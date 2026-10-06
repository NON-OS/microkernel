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

//! The proof, its own check, and the file it leaves. The blinding entropy is
//! fresh for this proof and refused when the kernel has none; it and the
//! witness are wiped as soon as the prover returns, whatever it returned. The
//! check is made on the output file read back, as a verifier reads it, so
//! what is saved is what was checked.

use alloc::string::String;

use nonos_device_attest::{prove, verify};
use nonos_libc::crypto_random;

use super::state::{Outcome, Step, Work};
use super::text::{hex, prover};
use super::{errno, store};
use crate::assemble::{decode, encode, wipe, wipe_bytes};

const ENTROPY: usize = 64;

fn missing() -> String {
    String::from("A step before this one did not finish")
}

pub fn proof(w: &mut Work) -> Outcome {
    let st = w.statement.as_ref().ok_or_else(missing)?;
    let mut witness = w.witness.take().ok_or_else(missing)?;
    let mut entropy = [0u8; ENTROPY];
    let got = crypto_random(entropy.as_mut_ptr(), ENTROPY);
    let made = if got == ENTROPY as i64 {
        prove(st, &witness, &entropy).map_err(|e| alloc::format!("Proof: {}", prover(&e)))
    } else {
        Err(String::from("Proof: the kernel gave no entropy to hide the secret with"))
    };
    wipe_bytes(&mut entropy);
    wipe(&mut witness);
    w.proof = Some(made?);
    Ok(("Proof: made".into(), Some(Step::Verify)))
}

pub fn check(w: &mut Work) -> Outcome {
    let st = w.statement.as_ref().ok_or_else(missing)?;
    let p = w.proof.as_ref().ok_or_else(missing)?;
    let bytes = encode(st, p).map_err(|r| alloc::format!("Check: {}", r.why()))?;
    let back =
        decode(&bytes).ok_or_else(|| String::from("Check: the output does not read back"))?;
    if back.statement != *st {
        return Err("Check: the output reads back as another statement".into());
    }
    verify(&back.statement, &back.proof).map_err(|e| alloc::format!("Check: {}", prover(&e)))?;
    w.output = Some(bytes);
    Ok(("Check: the saved form verifies against its statement alone".into(), Some(Step::Write)))
}

pub fn save(w: &mut Work) -> Outcome {
    let st = w.statement.as_ref().ok_or_else(missing)?;
    let bytes = w.output.as_ref().ok_or_else(missing)?;
    let name = store::save(bytes).map_err(|e| alloc::format!("Save: {}", errno::save(e)))?;
    let tag = hex(&st.tag[0].to_u64().to_le_bytes());
    Ok((alloc::format!("Saved {name}, tag {tag}..., {} bytes", bytes.len()), None))
}
