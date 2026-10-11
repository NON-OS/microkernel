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

//! A first PROVE: answered at once with its job id, then proven, with STATUS
//! and GET answered between the stages. EINVAL for a body that is not exactly
//! listing_id and policy; ENOMEM, and exit, when the heap was not had.

use nonos_libc::{mk_exit, mk_ipc_reply};
use nonos_proof_abi::wire::{reply, Header, ProveRequest, Stage, PROVE_MAGIC};

use super::answer::drain;
use super::state::Job;

const EINVAL: i32 = -22;
const ENOMEM: i32 = -12;

pub fn start(h: &Header, body: &[u8], sender: u32, held: bool, buf: &mut [u8]) -> Option<Job> {
    let say = |status: i32, out: &[u8]| {
        let tx = reply(PROVE_MAGIC, h, status, out);
        let _ = mk_ipc_reply(sender, tx.as_ptr(), tx.len());
    };
    let Some(req) = ProveRequest::decode(body).filter(|_| held) else {
        say(if held { EINVAL } else { ENOMEM }, &[]);
        if !held {
            mk_exit(1);
        }
        return None;
    };
    let (listing_id, policy) = (*req.listing_id, *req.policy);
    let mut j = Job::new(h.id, sender);
    say(0, &j.id.to_le_bytes());
    let req = ProveRequest { listing_id: &listing_id, policy: &policy };
    let made = super::run::prove(&req, &mut |stage| {
        j.stage = stage;
        drain(&mut j, buf);
    });
    match made {
        Ok(out) => (j.stage, j.out) = (Stage::Done, out),
        Err(status) => (j.stage, j.status) = (Stage::Failed, status),
    }
    Some(j)
}
