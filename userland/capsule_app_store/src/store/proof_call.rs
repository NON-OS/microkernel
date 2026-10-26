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

//! One call to app.prove.job, in nonos_proof_abi::wire's NPRV frames: PROVE,
//! STATUS or GET. A GET's reply holds container bytes, so the buffer is
//! wiped once read. The pure files' copies of the wire's numbers are held
//! to the wire here, so a change there fails this build.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;
use nonos_proof_abi::wire::{reply_body, request, ReplyError, Stage, FRAME_MAX, PROVE_MAGIC};
use nonos_proof_abi::wire::{GET_MAX, NOT_ENROLLED, OUTSIDE, POLICY, PROVER, STALE_REGISTRY};
use zeroize::Zeroize;

use super::super::market::next_id;
use super::{proof_get as get, proof_said as said, proof_seq as seq};

const _: () = assert!(get::GET_MAX == GET_MAX);
const _: () = assert!(get::MAX_BYTES as usize == nonos_proof_abi::MAX_BYTES);
const _: () = assert!(said::STALE_REGISTRY == STALE_REGISTRY && said::POLICY == POLICY);
const _: () = assert!(said::NOT_ENROLLED == NOT_ENROLLED && said::PROVER == PROVER);
const _: () = assert!(said::OUTSIDE == OUTSIDE);
const _: () = assert!(seq::READING == Stage::Reading as u8 && seq::DONE == Stage::Done as u8);
const _: () =
    assert!(seq::DEVICE == Stage::Device as u8 && seq::ISOLATION == Stage::Isolation as u8);
const _: () = assert!(seq::QUEUED == Stage::Queued as u8);

/// A reply that is not the job's, or not this call's.
const EBADMSG: i32 = -74;
/// The kernel's ERRNO_TIMEDOUT: the call's wait ran out, the job still owes it.
pub const ETIMEDOUT: i32 = -110;

/// The reply's body after its status, or the status, or the call's errno.
pub fn call(port: u32, op: u16, body: &[u8], wait_ms: u64) -> Result<Vec<u8>, i32> {
    let id = next_id();
    let tx = request(PROVE_MAGIC, op, id, body);
    let mut rx = vec![0u8; FRAME_MAX];
    let (out, cap) = (rx.as_mut_ptr(), rx.len());
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), out, cap, wait_ms.max(1));
    let got = match usize::try_from(n) {
        Err(_) => Err(i32::try_from(n).unwrap_or(EBADMSG)),
        Ok(n) => match reply_body(rx.get(..n).unwrap_or(&[]), PROVE_MAGIC, op, id) {
            Ok(b) => Ok(b.to_vec()),
            Err(ReplyError::Status(s)) => Err(s),
            Err(_) => Err(EBADMSG),
        },
    };
    rx.zeroize();
    got
}

/// STATUS's body: the stage, then the total once Done.
pub fn status(body: &[u8]) -> Result<(u8, u32), i32> {
    match body {
        [stage, t0, t1, t2, t3] => Ok((*stage, u32::from_le_bytes([*t0, *t1, *t2, *t3]))),
        _ => Err(EBADMSG),
    }
}
