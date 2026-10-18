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

//! Every entropy source there is, hashed into one seed. RDSEED, RDRAND and
//! the entropy capsule's held bytes go into one SHA-256 with cycle-counter
//! jitter and this CPU's stack and counter, so a weak source is covered by
//! the others. It fails closed below 32 bytes counted, the capsule's counting
//! for at most half (K9).

use alloc::vec::Vec;

use super::super::capsule_bytes;
use super::super::credit::enough;
use super::super::error::EntropyError;
use super::super::hardware::{cpu_entropy64, cpu_random64, read_cycle_counter};
use super::super::state::ENTROPY_COUNTER;
use crate::crypto::hash::sha256;
use core::sync::atomic::Ordering;

const DOMAIN: &[u8] = b"NONOS seed pool v1";
const WORDS: usize = 4;
const JITTER: usize = 16;

pub fn collect_seed_entropy_secure() -> Result<[u8; 32], EntropyError> {
    // Room for every input, so no push reallocates and frees a copy unzeroed.
    let mut pool = Vec::with_capacity(512);
    pool.extend_from_slice(DOMAIN);
    let mut hardware = 0usize;
    for source in [cpu_entropy64 as fn() -> Option<u64>, cpu_random64] {
        for _ in 0..WORDS {
            if let Some(v) = source() {
                pool.extend_from_slice(&v.to_le_bytes());
                hardware += 8;
            }
        }
    }
    let mut capsule = 0usize;
    if let Some((held, counted)) = capsule_bytes::take() {
        pool.extend_from_slice(&held);
        capsule = counted;
    }
    for _ in 0..JITTER {
        let t1 = read_cycle_counter();
        for _ in 0..((t1 & 0x1F) + 1) {
            core::hint::spin_loop();
        }
        pool.extend_from_slice(&read_cycle_counter().wrapping_sub(t1).to_le_bytes());
    }
    let counter = ENTROPY_COUNTER.fetch_add(0xA7B3_C5D9_E1F4_2680, Ordering::SeqCst);
    pool.extend_from_slice(&crate::arch::stack_pointer().to_le_bytes());
    pool.extend_from_slice(&counter.to_le_bytes());
    let seed = sha256(&pool);
    for b in pool.iter_mut() {
        // SAFETY: `b` is a live, exclusively borrowed byte of `pool`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    // Jitter and addresses stir the pool; they are never counted as entropy.
    if !enough(hardware, capsule) {
        return Err(EntropyError::InsufficientEntropy);
    }
    Ok(seed)
}
