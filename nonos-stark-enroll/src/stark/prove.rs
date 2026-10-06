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

use nonos_attest_path::{context_digest, encode_v4, Kind};
use stark_proofs::attest::{prove, Error, Statement, POINTS};
use stark_proofs::crypto::stark::field::Fp;

use super::check::check_v4;
use super::witness::{witness_of, words};
use crate::io::os_random;

/* Fresh blinding draws before a slot is given up, as the prover's own rank rule. */
const ATTEMPTS: usize = 3;

/// The v4 trailer for one enrolled slot: its v3 `path` and a STARK proof of the
/// same slot, checked by the gates' verifier before it is returned.
pub fn prove_v4(root: &[u8; 32], kind: Kind, ctx: &[u8], path: &[u8]) -> Result<Vec<u8>, String> {
    let root_words = words(root).ok_or("root is not four canonical words")?;
    let digest = context_digest(ctx).ok_or("context longer than a u32 counts")?;
    let st = Statement::new(root_words.map(Fp::from_u64), digest, kind as u64)
        .ok_or("a kind that is never proven")?;
    agree(&st, root_words, ctx, kind)?;
    let w = witness_of(path).ok_or("path is not a depth 8 v3 trailer")?;
    let mut last = String::new();
    for _ in 0..ATTEMPTS {
        let mut entropy = [0u8; 64];
        entropy[..32].copy_from_slice(&os_random());
        entropy[32..].copy_from_slice(&os_random());
        match prove(&st, &w, &entropy, POINTS[0]) {
            Ok(proof) => {
                let t = encode_v4(kind, path, &proof).ok_or("proof does not fit a v4 trailer")?;
                check_v4(root, kind, ctx, &t)?;
                return Ok(t);
            }
            Err(Error::Rank(why)) => last = why,
            Err(e) => return Err(format!("prover refused: {e:?}")),
        }
    }
    Err(format!("no zero-knowledge certificate after {ATTEMPTS} draws: {last}"))
}

/*
 * The prover's words and the gates' words come from two implementations. A
 * proof is only made when they agree, so a drift between them is an enrollment
 * error, never a kernel that refuses its own boot.
 */
fn agree(st: &Statement, root: [u64; 4], ctx: &[u8], kind: Kind) -> Result<(), String> {
    let gate = nox_verify::attest::words(root, ctx, kind as u64).ok_or("no gate words")?;
    let prover: Vec<u64> = st.words().iter().map(|f| f.value()).collect();
    if prover != gate {
        return Err(format!("prover words {prover:?} differ from the gate's {gate:?}"));
    }
    Ok(())
}
