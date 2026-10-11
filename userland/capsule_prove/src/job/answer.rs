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

//! Answers to everything but a first PROVE: HEALTH, STATUS and GET for the
//! job's owner, EBUSY to another PROVE, ENOENT to anyone else's STATUS or GET.

use alloc::vec::Vec;

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};
use nonos_proof_abi::wire::{
    job_of, reply, status_body, Header, Stage, DELIVER, GET, GET_MAX, HEALTH, PROVE, PROVE_MAGIC,
    STATUS,
};

use super::state::Job;

const EBUSY: i32 = -16;
const ENOENT: i32 = -2;
const EINVAL: i32 = -22;

pub fn answer(job: Option<&mut Job>, sender: u32, h: &Header, body: &[u8]) -> Vec<u8> {
    let (status, out): (i32, Vec<u8>) = match (h.op, job) {
        (HEALTH, _) => (0, Vec::new()),
        (PROVE | DELIVER, _) => (EBUSY, Vec::new()),
        (STATUS | GET, Some(j))
            if job_of(body).is_some_and(|(id, _)| id == j.id) && sender == j.owner =>
        {
            match (h.op, job_of(body)) {
                (STATUS, Some((_, None))) => (j.status, status_body(j.stage, j.total()).to_vec()),
                (GET, Some((_, Some(at)))) if j.stage == Stage::Done => slice(j, at as usize),
                _ => (EINVAL, Vec::new()),
            }
        }
        (STATUS | GET, _) => (ENOENT, Vec::new()),
        _ => (EINVAL, Vec::new()),
    };
    reply(PROVE_MAGIC, h, status, &out)
}

fn slice(j: &mut Job, at: usize) -> (i32, Vec<u8>) {
    let Some(rest) = j.out.get(at..) else {
        return (EINVAL, Vec::new());
    };
    let end = rest.len().min(GET_MAX);
    j.fetched |= at + end == j.out.len();
    (0, rest[..end].to_vec())
}

/// While a job runs: answer what has queued up, without waiting.
pub fn drain(job: &mut Job, buf: &mut [u8]) {
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), buf.len(), 0, &mut sender);
        let Some(n) = usize::try_from(n).ok().filter(|&n| n > 0 && sender != 0) else {
            return;
        };
        if let Some((h, body)) = Header::parse(&buf[..n], PROVE_MAGIC) {
            let tx = answer(Some(job), sender, &h, body);
            let _ = mk_ipc_reply(sender, tx.as_ptr(), tx.len());
        }
    }
}
