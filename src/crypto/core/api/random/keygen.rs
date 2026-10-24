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

//! A 32-byte key: the kernel generator, whose seed holds every source's
//! entropy, hashed with CPU RNG words and timing, which only stir; refused unseeded.

use super::platform::{
    get_stack_pointer, random64_or_counter, read_cycle_counter, read_pit_counter,
    read_rtc_timestamp, secure_random64, KEYGEN_COUNTER,
};
use crate::crypto::util::rng;
use crate::crypto::{blake3_hash, CryptoError::InsufficientEntropy, CryptoResult};
use core::sync::atomic::{compiler_fence, Ordering};

// The generator's 64, two words per round for 16 rounds, four more.
const POOL: usize = 64 + 16 * 16 + 4 * 8;

struct Pool {
    bytes: [u8; POOL],
    at: usize,
}

impl Pool {
    fn put(&mut self, v: u64) {
        self.bytes[self.at..self.at + 8].copy_from_slice(&v.to_le_bytes());
        self.at += 8;
    }
}

/// Spin for a PIT-dependent count, then return the cycles it took.
fn jitter(base: u32) -> u64 {
    let start = read_cycle_counter();
    for _ in 0..((read_pit_counter() as u32) & 0x3F).wrapping_add(base) {
        core::hint::spin_loop();
    }
    read_cycle_counter().wrapping_sub(start)
}

pub fn generate_secure_key() -> CryptoResult<[u8; 32]> {
    if secure_random64().is_none() {
        crate::log_warn!("crypto: no CPU RNG; this key rests on the kernel generator's seed");
    }
    let mut p = Pool { bytes: [0u8; POOL], at: 64 };
    rng::fill_random_bytes_secure(&mut p.bytes[..64]).map_err(|_| InsufficientEntropy)?;
    let start = read_cycle_counter();
    for i in 0..16u32 {
        p.put(random64_or_counter());
        p.put(jitter(i * 10) ^ (read_pit_counter() as u64) << 48);
    }
    p.put(get_stack_pointer() ^ p.bytes.as_ptr() as u64);
    p.put(read_rtc_timestamp());
    p.put(crate::time::timestamp_millis() ^ (read_pit_counter() as u64) << 32);
    let counter = KEYGEN_COUNTER.fetch_add(0xA3B7_C1D5_E9F2_4680, Ordering::SeqCst);
    p.put(counter ^ read_cycle_counter().wrapping_sub(start));
    let key = blake3_hash(&p.bytes[..p.at]);
    for byte in p.bytes.iter_mut() {
        // SAFETY: `byte` is a live, exclusively borrowed byte of the pool.
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    compiler_fence(Ordering::SeqCst);
    Ok(key)
}
