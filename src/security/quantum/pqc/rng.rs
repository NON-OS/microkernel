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
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use spin::Mutex;

pub struct QuantumRng {
    pool: Mutex<Vec<u8>>,
    health_checks: AtomicU32,
    last_check: AtomicU64,
    healthy: AtomicBool,
}

impl QuantumRng {
    /// The pool is only what `health_check` measures. With no randomness it
    /// stays empty and the generator starts unhealthy.
    pub fn new() -> Self {
        let mut pool = alloc::vec![0u8; 4096];
        let drawn = crate::crypto::fill_random(&mut pool).is_ok();
        if !drawn {
            pool.clear();
        }
        Self {
            pool: Mutex::new(pool),
            health_checks: AtomicU32::new(0),
            last_check: AtomicU64::new(crate::time::timestamp_millis()),
            healthy: AtomicBool::new(drawn),
        }
    }

    /// `n` fresh bytes from the seeded RNG, or `None` when it cannot answer.
    pub fn gen_bytes(&self, n: usize) -> Option<Vec<u8>> {
        let mut out = alloc::vec![0u8; n];
        crate::crypto::fill_random(&mut out).ok()?;
        Some(out)
    }

    pub fn health_check(&self) -> bool {
        self.health_checks.fetch_add(1, Ordering::Relaxed);
        let pool = self.pool.lock();
        let entropy = crate::crypto::estimate_entropy(&pool);
        self.last_check.store(crate::time::timestamp_millis(), Ordering::Relaxed);
        let healthy = entropy > 7.5;
        self.healthy.store(healthy, Ordering::Relaxed);
        healthy
    }

    pub fn is_healthy(&self) -> bool {
        self.healthy.load(Ordering::Relaxed)
    }
}
