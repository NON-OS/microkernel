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

//! The receipt job, found and called: init spawns it on app.prove.job once
//! asked, and it answers the Terminal alone (capsule_prove receipt_serve.rs).

use alloc::string::String;

use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::{mk_ipc_call_timeout, mk_prove_job, mk_uptime_ms, mk_yield, PROVE_JOB_RECEIPT};

use super::receipt_frame::{reply, request};

const JOB: &[u8] = b"app.prove.job";
const READY_MS: i64 = 60_000;
const CALL_MS: u64 = 120_000;

/// A RECEIPT job asked of init and found on its port. A job just acked may
/// still be leaving, so a busy queue is asked again until READY_MS.
pub fn job() -> Result<u32, String> {
    let (start, mut asked) = (mk_uptime_ms(), false);
    loop {
        asked = asked || mk_prove_job(PROVE_JOB_RECEIPT) >= 0;
        match lookup_service(JOB).filter(|_| asked) {
            Some(peer) => return Ok(peer.port),
            None if mk_uptime_ms() - start < READY_MS => {
                let _ = mk_yield();
            }
            None => return Err("no receipt job answered".into()),
        }
    }
}

/// The status and body of the job's answer to one frame; None for none.
pub fn status<'a>(
    port: u32,
    op: u16,
    id: u32,
    body: &[u8],
    rx: &'a mut [u8],
) -> Option<(i32, &'a [u8])> {
    let tx = request(op, id, body);
    let n =
        mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len(), CALL_MS);
    reply(rx.get(..usize::try_from(n).ok()?)?, op, id)
}

/// The body of the job's answer to one frame, or why there is none.
pub fn call<'a>(
    port: u32,
    op: u16,
    id: u32,
    body: &[u8],
    rx: &'a mut [u8],
) -> Result<&'a [u8], String> {
    match status(port, op, id, body, rx) {
        Some((0, out)) => Ok(out),
        Some((s, _)) => Err(alloc::format!("op {op}: refused ({s})")),
        None => Err(alloc::format!("op {op}: no answer")),
    }
}
