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

//! The kernel's ask for the entropy capsule's bytes, never waited on (K9).
//! An ask is sent and the caller goes on; replies that have landed are
//! drained on a later call and fed through the health tests into the
//! accumulator every seed draws from. Called from a syscall, with no lock
//! held, so no IPC happens under an RNG's lock or before the capsule runs.

use spin::Mutex;

use super::super::protocol::{decode_response, encode_request, MAX_RANDOM_BYTES, OP_GET_RANDOM};
use super::super::state::shared_state;
use super::ask_rule::{answers, due, Asked};
use super::seq::next_request_id;
use super::transport::REPLY_INBOX;
use crate::crypto::util::rng::{capsule_bytes_wanted, feed_capsule_bytes};
use crate::services::lifecycle::transport_nowait::{landed, post};

const ASK_BYTES: u32 = 256;
const _: () = assert!(ASK_BYTES <= MAX_RANDOM_BYTES);
const SENDER_NAME: &str = "kernel.entropy";

static ASKED: Mutex<Option<Asked>> = Mutex::new(None);

/// Drain the replies that have landed, then ask again when due.
pub fn poll() {
    let capsule = shared_state();
    // Taken under the generation the ask out went to; with none out, the
    // capsule's own, and ask_rule::answers refuses what nothing asked for.
    loop {
        let asked_gen = ASKED.lock().map_or(capsule.generation(), |a| a.generation);
        let Some(got) = landed(REPLY_INBOX, capsule, asked_gen) else { break };
        let Ok(reply) = got else { continue };
        let Some(resp) = decode_response(&reply.data) else { continue };
        {
            let mut asked = ASKED.lock();
            if !answers(*asked, resp.request_id, capsule.generation()) {
                continue;
            }
            *asked = None;
        }
        if resp.status == 0 && resp.body.len() == ASK_BYTES as usize {
            feed_capsule_bytes(resp.body);
        }
    }
    let (now, wanted) = (crate::time::timestamp_millis(), capsule_bytes_wanted());
    let id = {
        let mut asked = ASKED.lock();
        if !due(*asked, now, capsule.generation(), capsule.is_alive(), wanted) {
            return;
        }
        let id = next_request_id();
        *asked = Some(Asked { id, generation: capsule.generation(), at_ms: now });
        id
    };
    let frame = encode_request(OP_GET_RANDOM, 0, id, &ASK_BYTES.to_le_bytes());
    // Not sent: the ask stays out until GIVE_UP_MS, then is sent again.
    let _ = post(&frame, SENDER_NAME, capsule);
}
