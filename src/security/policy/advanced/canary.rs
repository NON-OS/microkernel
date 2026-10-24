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

//! Stack canaries.

extern crate alloc;

use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::RwLock;

pub struct StackCanary {
    global: AtomicU64,
    per_cpu: RwLock<BTreeMap<u32, u64>>,
    rotations: AtomicU64,
}

impl StackCanary {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Self {
            global: AtomicU64::new(Self::gen_canary().ok_or("no randomness for a canary")?),
            per_cpu: RwLock::new(BTreeMap::new()),
            rotations: AtomicU64::new(0),
        })
    }
    /// The CPU's random instruction, else the seeded RNG. With neither there
    /// is no canary: one made from the cycle counter could be guessed.
    fn gen_canary() -> Option<u64> {
        crate::arch::cpu_random::random_u64()
            .or_else(|| crate::crypto::rng::random_u64_secure().ok())
    }
    pub fn get(&self, cpu: u32) -> u64 {
        let lock = self.per_cpu.read();
        if let Some(c) = lock.get(&cpu) {
            *c
        } else {
            self.global.load(Ordering::Relaxed)
        }
    }
    /// Draws every new value first; with no randomness nothing changes and
    /// the rotation is refused.
    pub fn rotate(&self) -> Result<(), &'static str> {
        let none = "no randomness for a canary";
        let new = Self::gen_canary().ok_or(none)?;
        let mut lock = self.per_cpu.write();
        let mut fresh = alloc::vec::Vec::with_capacity(lock.len());
        for _ in 0..lock.len() {
            fresh.push(Self::gen_canary().ok_or(none)? ^ new);
        }
        self.global.store(new, Ordering::Release);
        for ((_, c), v) in lock.iter_mut().zip(fresh) {
            *c = v;
        }
        self.rotations.fetch_add(1, Ordering::Release);
        Ok(())
    }
    pub fn verify(&self, cpu: u32, value: u64) -> bool {
        value == self.get(cpu)
    }
}
