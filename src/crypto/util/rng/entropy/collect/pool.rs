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

//! Every entropy source there is, hashed into one seed.
//!
//! A seed from one source is only as good as that source. virtio-rng, RDSEED
//! and RDRAND are each drawn when present and all go into one SHA-256 with
//! cycle-counter jitter and this CPU's stack and counter, so a weak source is
//! covered by the others. It fails closed below 32 bytes from hardware.

use alloc::vec::Vec;

use super::super::error::EntropyError;
use super::super::hardware::{cpu_entropy64, cpu_random64, read_cycle_counter};
use super::super::state::ENTROPY_COUNTER;
use crate::crypto::hash::sha256;
use crate::drivers::virtio_rng;
use core::sync::atomic::Ordering;

const DOMAIN: &[u8] = b"NONOS seed pool v1";
const WORDS: usize = 4;
const JITTER: usize = 16;

pub fn collect_seed_entropy_secure() -> Result<[u8; 32], EntropyError> {
    let mut pool = Vec::with_capacity(256);
    pool.extend_from_slice(DOMAIN);
    let mut hardware = 0usize;
    let mut device = [0u8; 32];
    if virtio_rng::is_available() && virtio_rng::fill_random(&mut device).is_ok() {
        pool.extend_from_slice(&device);
        hardware += device.len();
    }
    for source in [cpu_entropy64 as fn() -> Option<u64>, cpu_random64] {
        for _ in 0..WORDS {
            if let Some(v) = source() {
                pool.extend_from_slice(&v.to_le_bytes());
                hardware += 8;
            }
        }
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
    if hardware < 32 {
        return Err(EntropyError::InsufficientEntropy);
    }
    Ok(seed)
}
