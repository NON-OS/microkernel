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

//! The call a step asks for, made, and the event it came to; None when
//! nothing came. A STATUS waits up to the job's quiet spell: the kernel
//! drops a reply that comes after its call gave up (recv_reply.rs), so a
//! short wait would leave the one outstanding STATUS (ruling 3) unanswered.

use nonos_libc::{mk_prove_job, mk_service_lookup, mk_uptime_ms, PROVE_JOB_PROVE};
use nonos_proof_abi::wire::{ProveRequest, GET, PROVE, STATUS};

use super::proof_call::{call, status, ETIMEDOUT};
use super::proof_event::{Call, Event};
use super::proof_state::Run;

const SERVICE: &[u8] = b"app.prove.job";
/// Init spawns the prover on the ask; not up in this long, it did not start.
const ASK_MS: i64 = 10_000;
/// The job's QUIET_MS (capsule_prove/src/job/exit_rule.rs).
const QUIET_MS: i64 = 10 * 60 * 1000;
/// PROVE is answered at once, and GET from a finished job.
const CALL_MS: u64 = 2_000;

pub fn perform(run: &mut Run, c: Call, market: u32) -> Option<Event> {
    let now = mk_uptime_ms();
    let job = run.job.to_le_bytes();
    Some(match c {
        Call::None => return None,
        Call::Policy => Event::Policy(run.boxed(market)),
        Call::AskJob => {
            run.late_ms = now.saturating_add(ASK_MS);
            Event::Job(mk_prove_job(PROVE_JOB_PROVE))
        }
        Call::Lookup => {
            let (mut port, mut pid) = (0u32, 0u32);
            let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
            run.port = port;
            return (rc == 0 && pid != 0).then_some(Event::Up);
        }
        Call::Prove => {
            let ask = ProveRequest { listing_id: &run.listing_key, policy: &run.policy };
            let got = call(run.port, PROVE, &ask.encode(), CALL_MS);
            let id = got.and_then(|b| b.try_into().map(u32::from_le_bytes).map_err(|_| -74));
            run.late_ms = now.saturating_add(QUIET_MS);
            Event::Proved(id.map(|id| run.job = id))
        }
        Call::Status => {
            match call(run.port, STATUS, &job, run.late_ms.saturating_sub(now).max(1) as u64) {
                Err(ETIMEDOUT) => Event::StatusLate,
                got => {
                    run.late_ms = mk_uptime_ms().saturating_add(QUIET_MS);
                    Event::Status(got.and_then(|b| status(&b)))
                }
            }
        }
        Call::Get(at) => {
            let got = call(run.port, GET, &[job, at.to_le_bytes()].concat(), CALL_MS);
            Event::Got(got.map(|chunk| run.take(chunk)))
        }
    })
}
