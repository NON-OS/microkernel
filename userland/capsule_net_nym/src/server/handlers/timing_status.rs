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

use crate::protocol::{E_OK, OP_TIMING_STATUS};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::message::PLAINTEXT_PER_PACKET;
use crate::mixnet::ceiling::ceilings;
use crate::mixnet::cover_clock::{profile_of, LOW_RATE};
use crate::sphinx::constants::{HEADER_SIZE, PAYLOAD_KEY_SEED_SIZE};
use crate::state;

const LEN: u32 = 20;

/// Hops in a route home, so in a reply block: three mixes and our gateway
/// (`topology::route` draws four nodes).
const BLOCK_HOPS: usize = 4;

/// A reply block as it travels in a top up (`surb/bytes.rs`): its key, the
/// header, the entry hop's address, a payload key seed a hop.
const BLOCK_BYTES: usize = 16 + HEADER_SIZE + 32 + BLOCK_HOPS * PAYLOAD_KEY_SEED_SIZE;

pub fn handle(pid: u32, req: &Request, tx: &mut [u8]) {
    let policy = state::timing_policy();
    let off = 20usize;
    tx[off..off + 2].copy_from_slice(&policy.cover_burst.to_le_bytes());
    tx[off + 2..off + 4].copy_from_slice(&policy.delay_jitter_ms.to_le_bytes());
    // The first of the four reserved bytes names the cover profile (0 the
    // low rate, 1 Nym's); the last field is when the sender's next tick falls.
    tx[off + 4..off + 8].fill(0);
    tx[off + 4] = crate::server::profile_code();
    tx[off + 8..off + 16].copy_from_slice(&crate::server::next_tick_ms().to_le_bytes());
    // The profile's ceilings, KiB a second out and in: what Settings shows.
    let profile = profile_of(crate::server::profile_code()).unwrap_or(LOW_RATE);
    let (out, inbound) =
        ceilings(profile, PLAINTEXT_PER_PACKET as u64, (PLAINTEXT_PER_PACKET / BLOCK_BYTES) as u64);
    tx[off + 16..off + 18].copy_from_slice(&out.to_le_bytes());
    tx[off + 18..off + 20].copy_from_slice(&inbound.to_le_bytes());
    respond(pid, OP_TIMING_STATUS, E_OK, req.request_id, LEN, tx);
}
