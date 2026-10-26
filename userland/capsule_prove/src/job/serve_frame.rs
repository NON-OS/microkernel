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

//! One frame the serving loop received (serve.rs): the worker looked at
//! first, so a STATUS reads its stage of now; the owner heard; then an
//! answer by the frame's request id, which says whether the call
//! took it, or the job started by the frame init spawned the prover for.

use alloc::vec::Vec;

use nonos_libc::ipc::{mk_ipc_reply_to, IpcReqMeta};
use nonos_proof_abi::wire::{Header, PROVE_MAGIC};

use super::answer::answer;
use super::poll::poll;
use super::serve::{now, Served};
use super::state::Job;

pub fn frame(
    s: &mut Served,
    (buf, n): (&mut Vec<u8>, usize),
    m: IpcReqMeta,
    (held, bound): (bool, Option<(u16, u32)>),
) {
    let sender = m.sender;
    s.last = now();
    poll(s.job.as_mut(), &mut s.finish, s.last);
    if let Some(j) = s.job.as_mut().filter(|j| j.owner == sender) {
        j.heard(s.last);
    }
    let Some((h, body)) = Header::parse(&buf[..n], PROVE_MAGIC) else {
        return;
    };
    if s.job.is_some() || Some((h.op, sender)) != bound {
        let tx = answer(s.job.as_mut(), sender, &h, body);
        let rc = mk_ipc_reply_to(m.request, tx.as_ptr(), tx.len());
        if let Some(j) = s.job.as_mut() {
            j.replied(rc);
        }
        return;
    }
    let body = body.to_vec();
    // The failure's 30 s run from when it failed (exit_rule.rs).
    let started = super::start::start(&h, &body, sender, held, buf);
    (s.job, s.finish) = match started {
        Some((j, f)) => (Some(Job { asker_ms: now(), ..j }), f),
        None => (None, None),
    };
    s.last = now();
}
