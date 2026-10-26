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
//! then proven on the worker thread (worker.rs) while this one answers
//! STATUS and GET; exit when exit_rule.rs says: the container fetched, a
//! failure told or left unread, or a quiet spell. Exit zeroizes its RAM.

use alloc::vec;

use nonos_libc::{mk_exit, mk_ipc_recv_from, mk_time_millis, mk_yield};
use nonos_proof_abi::wire::FRAME_MAX;

use super::exit_rule::{exits, wait_ms};
use super::finish::Finish;
use super::poll::poll;
use super::state::Job;

/// What the loop holds: the job, what the worker's result becomes, and
/// when a request last came.
pub struct Served {
    pub job: Option<Job>,
    pub finish: Option<Finish>,
    pub last: u64,
}

/// `held`: the prover's heap was had. Without it every PROVE is ENOMEM, and
/// the job exits at once rather than fail halfway through a proof. `bound`:
/// the op and the pid init spawned it for; no other first frame starts it.
pub fn serve(held: bool, bound: Option<(u16, u32)>) -> ! {
    if bound.is_none() {
        mk_exit(1);
    }
    let mut buf = vec![0u8; FRAME_MAX];
    let mut s = Served { job: None, finish: None, last: now() };
    loop {
        // A job that ends now starts its quiet spell now, as if asked.
        if poll(s.job.as_mut(), &mut s.finish, now()) {
            s.last = now();
        }
        let seen = s.job.as_ref().map(Job::seen);
        if exits(seen, now(), s.last) {
            mk_exit(0);
        }
        let mut sender = 0u32;
        let wait = wait_ms(seen, now(), s.last);
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), buf.len(), wait, &mut sender);
        match usize::try_from(n).ok().filter(|&n| n > 0 && sender != 0) {
            Some(n) => super::serve_frame::frame(&mut s, (&mut buf, n), sender, (held, bound)),
            None => {
                mk_yield();
            }
        }
    }
}

pub fn now() -> u64 {
    u64::try_from(mk_time_millis()).unwrap_or(0)
}
