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

//! The fast path for kernel-served randomness: a local ChaCha20 generator,
//! so no syscall waits on IPC. Each fill ends by replacing the key with
//! keystream no caller saw. Every RESEED_AFTER bytes the key is hashed with
//! a seed from every source, the entropy capsule's held bytes never
//! enough alone; the capsule is asked first, unlocked and never waited on.

use spin::Mutex;

use crate::crypto::hash::sha256;
use crate::crypto::symmetric::chacha20poly1305::chacha20_block;
use crate::crypto::util::rng::collect_seed_entropy_secure;

/// Output served between reseeds.
const RESEED_AFTER: u64 = 1024 * 1024;
const DOMAIN: &[u8] = b"NONOS fast rng reseed v1";

struct State {
    key: [u8; 32],
    counter: u32,
    since_reseed: u64,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

/// Fill `out` from the local generator, seeding it when due. False only
/// when it has never been seeded and no seed can be made now; the caller
/// then falls to the hardware RNG.
pub fn fill(out: &mut [u8]) -> bool {
    super::client::poll();
    let mut guard = STATE.lock();
    if guard.as_ref().map_or(true, |s| s.since_reseed >= RESEED_AFTER) {
        // A failed reseed keeps serving on the ratcheted key.
        if let Ok(mut seed) = collect_seed_entropy_secure() {
            let mut input = [0u8; 24 + 64];
            input[..24].copy_from_slice(DOMAIN);
            input[24..56].copy_from_slice(guard.as_ref().map_or(&[0u8; 32], |s| &s.key));
            input[56..].copy_from_slice(&seed);
            *guard = Some(State { key: sha256(&input), counter: 0, since_reseed: 0 });
            input.fill(0);
            seed.fill(0);
        }
    }
    let Some(state) = guard.as_mut() else {
        return false;
    };
    let nonce = [0u8; 12];
    let mut block = [0u8; 64];
    for chunk in out.chunks_mut(64) {
        chacha20_block(&state.key, &nonce, state.counter, &mut block);
        state.counter = state.counter.wrapping_add(1);
        chunk.copy_from_slice(&block[..chunk.len()]);
    }
    chacha20_block(&state.key, &nonce, state.counter, &mut block);
    state.counter = state.counter.wrapping_add(1);
    state.key.copy_from_slice(&block[..32]);
    block.fill(0);
    state.since_reseed = state.since_reseed.saturating_add(out.len() as u64);
    true
}
