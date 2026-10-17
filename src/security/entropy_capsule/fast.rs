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

//! The fast path for kernel-served randomness.
//!
//! The entropy capsule stays the root of the entropy story: it is where seed
//! material comes from and the only thing that can say a seed existed. It must
//! not be a per-byte dependency: a round trip per `crypto_random` put a context
//! switch inside the hottest primitive, millions of messages a minute into one
//! inbox on one core under a networking load, and the machine queued behind.
//!
//! So: a ChaCha20 generator, seeded from the capsule, serving locally. Fast key
//! erasure — every fill ends by replacing the key with fresh keystream, so a
//! later key compromise does not expose earlier output. Reseeded from the
//! capsule after `RESEED_AFTER` bytes, which keeps the capsule the recurring
//! source without putting it on the per-call path.

use spin::Mutex;

use crate::crypto::symmetric::chacha20poly1305::chacha20_block;

/// Output served between capsule reseeds. One reseed per this many bytes is
/// one IPC per megabyte instead of one per call.
const RESEED_AFTER: u64 = 1024 * 1024;

struct State {
    key: [u8; 32],
    counter: u32,
    since_reseed: u64,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

/// Fill `out` from the local generator, seeding or reseeding from the entropy
/// capsule when due. Returns false only when no seed has ever been obtained
/// and the capsule cannot provide one now; the caller keeps its existing
/// hardware-RNG fallback for that case.
pub fn fill(out: &mut [u8]) -> bool {
    let due = |s: &Option<State>| s.as_ref().is_none_or(|s| s.since_reseed >= RESEED_AFTER);
    // The capsule is asked with STATE let go: a caller paused in that round
    // trip, or killed in it and never woken, would leave every other caller
    // spinning on it with interrupts masked (L81).
    let reseed_due = due(&STATE.lock());
    if reseed_due {
        let mut seed = [0u8; 32];
        if let Ok(n) = super::client::get_random(&mut seed) {
            let mut guard = STATE.lock();
            // Another caller's reseed may have landed meanwhile; one is enough.
            if n == seed.len() && due(&guard) {
                *guard = Some(State { key: seed, counter: 0, since_reseed: 0 });
            }
        }
        // A failed reseed keeps serving on the ratcheted key rather than
        // stalling every caller. A failed first seed leaves no state, and the
        // binding below reports that so the caller falls to the hardware RNG.
        seed.fill(0);
    }

    let mut guard = STATE.lock();
    let Some(state) = guard.as_mut() else {
        return false;
    };
    let nonce = [0u8; 12];
    let mut block = [0u8; 64];
    let mut done = 0usize;
    while done < out.len() {
        chacha20_block(&state.key, &nonce, state.counter, &mut block);
        state.counter = state.counter.wrapping_add(1);
        let take = (out.len() - done).min(block.len());
        out[done..done + take].copy_from_slice(&block[..take]);
        done += take;
    }

    // Fast key erasure: the next key is keystream the caller never saw, and
    // the old key does not survive the fill it produced.
    chacha20_block(&state.key, &nonce, state.counter, &mut block);
    state.counter = state.counter.wrapping_add(1);
    state.key.copy_from_slice(&block[..32]);
    block.fill(0);
    state.since_reseed = state.since_reseed.saturating_add(out.len() as u64);
    true
}
