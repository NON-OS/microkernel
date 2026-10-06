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

/*
 * One call to nonos.shield. Every call answers at once: the slow work
 * (opening, reading the chain, proving) runs as the service's one job, and
 * the wallet asks for its result on later ticks. A request that carried
 * words or a key is zeroed after it is sent.
 */

use alloc::string::String;
use alloc::vec;
use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};
use shield_wire::{decode_reply, encode_request, SERVICE};

const TIMEOUT_MS: u64 = 4000;
/* The service never answers more than this; a state reply cuts its oldest
 * history entries to fit and says how many. */
const RX_LEN: usize = shield_wire::MESSAGE_MAX;

pub const NO_SERVICE: &str = "No shield service is running on this machine.";
pub const NO_ANSWER: &str = "The shield service did not answer. Try again in a moment.";
/* Said while the wallet waits to ask a slow service again; not a failure. */
pub const STARTING: &str = "The shield service is still starting on this machine. The wallet \
     asks it again by itself; nothing needs doing.";

static SEQ: AtomicU32 = AtomicU32::new(1);

pub struct Answer {
    pub status: i32,
    pub body: String,
}

fn port() -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    (rc >= 0 && port != 0 && pid != 0).then_some(port)
}

fn wipe(b: &mut [u8]) {
    for x in b.iter_mut() {
        unsafe { core::ptr::write_volatile(x, 0) };
    }
}

pub fn call(op: u16, fields: &[&str]) -> Result<Answer, &'static str> {
    let port = port().ok_or(NO_SERVICE)?;
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let mut tx = encode_request(seq, op, fields);
    let mut rx = vec![0u8; RX_LEN];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    wipe(&mut tx);
    if rc < 8 {
        return Err(NO_ANSWER);
    }
    let reply = decode_reply(&rx[..rc as usize]).ok_or(NO_ANSWER)?;
    if reply.seq != seq {
        return Err(NO_ANSWER);
    }
    Ok(Answer { status: reply.status, body: String::from(reply.body) })
}
