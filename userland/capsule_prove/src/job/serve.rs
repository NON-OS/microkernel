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

//! The headless prover: one PROVE, answered at once with its job,
//! then proven; STATUS and GET between stages and after; exit once the
//! container is fetched, or after a quiet spell. Exit zeroizes its RAM.

use alloc::vec;

use nonos_libc::{mk_exit, mk_ipc_recv_from, mk_ipc_reply, mk_time_millis, mk_yield};
use nonos_proof_abi::wire::{Header, DELIVER, FRAME_MAX, PROVE, PROVE_MAGIC};

use super::answer::answer;
use super::state::Job;

/// No PROVE within this long of starting, or no request this long after the
/// last: the job was abandoned.
const QUIET_MS: u64 = 10 * 60 * 1000;

/// `held`: the prover's heap was had. Without it every PROVE is ENOMEM, and
/// the job exits at once rather than fail halfway through a proof.
pub fn serve(held: bool) -> ! {
    let mut buf = vec![0u8; FRAME_MAX];
    let mut job: Option<Job> = None;
    let mut last = now();
    loop {
        if job.as_ref().is_some_and(|j| j.fetched) || now().saturating_sub(last) > QUIET_MS {
            mk_exit(0);
        }
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), buf.len(), 0, &mut sender);
        let Some(n) = usize::try_from(n).ok().filter(|&n| n > 0 && sender != 0) else {
            mk_yield();
            continue;
        };
        last = now();
        let Some((h, body)) = Header::parse(&buf[..n], PROVE_MAGIC) else {
            continue;
        };
        if !(h.op == PROVE || h.op == DELIVER) || job.is_some() {
            let tx = answer(job.as_mut(), sender, &h, body);
            let _ = mk_ipc_reply(sender, tx.as_ptr(), tx.len());
            continue;
        }
        let body = body.to_vec();
        job = super::start::start(&h, &body, sender, held, &mut buf);
        last = now();
    }
}

fn now() -> u64 {
    u64::try_from(mk_time_millis()).unwrap_or(0)
}
