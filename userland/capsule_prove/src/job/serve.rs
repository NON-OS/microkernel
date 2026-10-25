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

//! The headless prover (plan/14): one PROVE, answered at once with its job,
//! then proven; STATUS and GET between stages and after; exit when
//! exit_rule.rs says: the container fetched, a failure told or left unread,
//! or a quiet spell. Exit zeroizes its RAM.

use alloc::vec;

use nonos_libc::{mk_exit, mk_ipc_recv_from, mk_ipc_reply, mk_time_millis, mk_yield};
use nonos_proof_abi::wire::{Header, FRAME_MAX, PROVE_MAGIC};

use super::answer::answer;
use super::exit_rule::{exits, wait_ms};
use super::state::Job;

/// `held`: the prover's heap was had. Without it every PROVE is ENOMEM, and
/// the job exits at once rather than fail halfway through a proof. `bound`:
/// the op and the pid init spawned it for; no other first frame starts it.
pub fn serve(held: bool, bound: Option<(u16, u32)>) -> ! {
    let Some((op, pid)) = bound else { mk_exit(1) };
    if op == nonos_proof_abi::wire::RECEIPT { super::receipt_serve::serve(pid, held) }
    let mut buf = vec![0u8; FRAME_MAX];
    let mut job: Option<Job> = None;
    let mut last = now();
    loop {
        let seen = job.as_ref().map(Job::seen);
        if exits(seen, now(), last) {
            mk_exit(0);
        }
        let mut sender = 0u32;
        let wait = wait_ms(seen, now(), last);
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), buf.len(), wait, &mut sender);
        let Some(n) = usize::try_from(n).ok().filter(|&n| n > 0 && sender != 0) else {
            mk_yield();
            continue;
        };
        last = now();
        if let Some(j) = job.as_mut().filter(|j| j.owner == sender) {
            j.asker_ms = last;
        }
        let Some((h, body)) = Header::parse(&buf[..n], PROVE_MAGIC) else {
            continue;
        };
        if job.is_some() || Some((h.op, sender)) != bound {
            let tx = answer(job.as_mut(), sender, &h, body);
            let _ = mk_ipc_reply(sender, tx.as_ptr(), tx.len());
            continue;
        }
        let body = body.to_vec();
        // The failure's 30 s run from when it failed (exit_rule.rs).
        job = super::start::start(&h, &body, sender, held, &mut buf)
            .map(|j| Job { asker_ms: now(), ..j });
        last = now();
    }
}

fn now() -> u64 {
    u64::try_from(mk_time_millis()).unwrap_or(0)
}
