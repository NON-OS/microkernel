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

//! One PROVE, in the plan's order: the sealed run for this policy's app, the
//! step-5 check, the device proof under the market scope, the isolation
//! proof, the container. Each stage is reported before it starts. This
//! gathers what the proofs need; the worker proves (worker.rs) and the
//! serving thread makes the container (finish.rs).

use alloc::vec;

use nonos_libc::{mk_debug, mk_seal_read, mk_time_millis};
use nonos_proof_abi::wire::{ProveRequest, Stage, PROVER};
use nonos_proof_abi::{app_id, RECORD_BYTES};

use crate::assemble::wipe_bytes;
use crate::market::{check, line, program, scope_and_context, split, Sealed, WITNESS_BYTES};

use super::finish::Finish;
use super::work::Work;

/// The kernel keeps at most this many measurements in a family.
const LIST_MAX: usize = 4096;
const DAY_MS: u64 = 86_400_000;

pub fn prove(req: &ProveRequest, stage: &mut dyn FnMut(Stage)) -> Result<(Work, Finish), i32> {
    stage(Stage::Reading);
    let s = sealed(req)?;
    let said = line(&s.hist);
    // Heard only on a kernel that grants this capsule Debug.
    let _ = mk_debug(said.as_ptr(), said.len());
    check(req.policy, req.listing_id, &s.record)?;
    let day = today()?;
    stage(Stage::Device);
    let device = super::device::inputs(day, scope_and_context(req.listing_id, day, &s.raw))?;
    let isolation = super::isolation::inputs(&s, req.policy)?;
    let st = isolation.st;
    let finish = Finish::Market { day, s, st, policy: *req.policy };
    Ok((Work { device, isolation: Some(isolation) }, finish))
}

/// The newest final record sealed for the policy's app, taken from the
/// kernel: ENOENT when there is none.
fn sealed(req: &ProveRequest) -> Result<Sealed, i32> {
    let id = app_id(req.policy, &program(req.policy));
    let mut buf = vec![0u8; RECORD_BYTES + WITNESS_BYTES + 32 * LIST_MAX];
    let n = mk_seal_read(&id, &mut buf);
    let s = usize::try_from(n).ok().and_then(|n| split(&buf[..n]));
    wipe_bytes(&mut buf);
    match s {
        Some(s) => Ok(s),
        None if n < 0 => Err(super::status::failed(n)),
        None => Err(PROVER),
    }
}

pub(super) fn today() -> Result<u64, i32> {
    u64::try_from(mk_time_millis()).ok().filter(|&ms| ms > 0).map(|ms| ms / DAY_MS).ok_or(PROVER)
}
