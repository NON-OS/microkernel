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

extern crate alloc;

pub use super::random_fill::{
    fill_bytes, fill_random, fill_random_bytes, secure_random_u32, secure_random_u8,
    try_fill_random,
};
pub use super::random_pool::{add_entropy, add_entropy_count, get_entropy_count, init};

use super::random_pool::consume_entropy;

pub fn secure_random_u64() -> u64 {
    // Retry the hardware source on each iteration. The previous code sampled
    // once and, on a single transient failure, spun forever WITHOUT retrying,
    // turning a momentary RDRAND/RDSEED hiccup into a permanent hang. Retrying
    // recovers from transient failures; only a sustained, genuinely dead
    // entropy source keeps looping, which is the correct fail-safe: fabricating
    // a value would hand out a predictable key/nonce/canary.
    loop {
        if let Ok(v) = try_secure_random_u64() {
            consume_entropy(64);
            return v;
        }
        core::hint::spin_loop();
    }
}

pub fn try_secure_random_u64() -> Result<u64, &'static str> {
    if let Some(v) = crate::arch::cpu_random::random_u64() {
        return Ok(v);
    }
    if let Some(v) = crate::arch::cpu_random::entropy_u64() {
        return Ok(v);
    }
    if let Ok(v) = try_virtio_rng() {
        return Ok(v);
    }
    Err("No hardware entropy source available")
}

fn try_virtio_rng() -> Result<u64, ()> {
    if crate::drivers::virtio_rng::is_available() {
        let mut buf = [0u8; 8];
        if crate::drivers::virtio_rng::fill_random(&mut buf).is_ok() {
            return Ok(u64::from_le_bytes(buf));
        }
    }
    Err(())
}
