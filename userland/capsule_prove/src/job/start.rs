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

//! A first PROVE or DELIVER: answered at once with its job id; what its
//! proofs need gathered, with STATUS and GET answered between the stages;
//! then the worker started on them (worker.rs). EINVAL for a body that does
//! not decode; ENOMEM, and exit, when the heap was not had.

use nonos_libc::{mk_exit, mk_ipc_reply};
use nonos_proof_abi::wire::{reply, Header, Stage, PROVE_MAGIC};

use super::ask::{ask, Ask};
use super::drain::drain;
use super::finish::Finish;
use super::state::Job;

const EINVAL: i32 = -22;
const ENOMEM: i32 = -12;

/// The job, and what the serving thread keeps while the worker proves; no
/// Finish when the job failed before the worker started.
pub fn start(
    h: &Header,
    body: &[u8],
    sender: u32,
    held: bool,
    buf: &mut [u8],
) -> Option<(Job, Option<Finish>)> {
    let say = |status: i32, out: &[u8]| {
        let tx = reply(PROVE_MAGIC, h, status, out);
        let _ = mk_ipc_reply(sender, tx.as_ptr(), tx.len());
    };
    let Some(a) = ask(h.op, body).filter(|_| held) else {
        say(if held { EINVAL } else { ENOMEM }, &[]);
        if !held {
            mk_exit(1);
        }
        return None;
    };
    let mut j = Job::new(h.id, sender);
    say(0, &j.id.to_le_bytes());
    let made = {
        let mut stage = |s: Stage| {
            j.stage = s;
            drain(&mut j, buf);
        };
        match a {
            Ask::Prove(r) => super::run::prove(&r, &mut stage),
            // The caller is the supervisor whose live family is delivered.
            Ask::Deliver(r) => super::deliver::deliver(&r, sender, &mut stage),
        }
    };
    let started = made.and_then(|(work, finish)| super::worker::start(work).map(|()| finish));
    match &started {
        Ok(_) => (j.stage, j.working) = (Stage::Device, true),
        Err(status) => (j.stage, j.status) = (Stage::Failed, *status),
    }
    Some((j, started.ok()))
}
