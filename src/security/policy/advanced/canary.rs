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
    pub fn new() -> Self {
        Self {
            global: AtomicU64::new(Self::gen_canary()),
            per_cpu: RwLock::new(BTreeMap::new()),
            rotations: AtomicU64::new(0),
        }
    }
    fn gen_canary() -> u64 {
        if let Some(v) = crate::arch::cpu_random::random_u64() {
            return v;
        }
        // No hardware generator on this CPU. A canary derived from the cycle
        // counter is weaker than a drawn one, but it still differs per boot and
        // per rotation, which is what a canary needs.
        crate::arch::read_time_counter() ^ 0xDEAD_BEEF_CAFE_BABE
    }
    pub fn get(&self, cpu: u32) -> u64 {
        let lock = self.per_cpu.read();
        if let Some(c) = lock.get(&cpu) {
            *c
        } else {
            self.global.load(Ordering::Relaxed)
        }
    }
    pub fn rotate(&self) {
        let new = Self::gen_canary();
        self.global.store(new, Ordering::Release);
        let mut lock = self.per_cpu.write();
        for (_, c) in lock.iter_mut() {
            *c = Self::gen_canary() ^ new;
        }
        self.rotations.fetch_add(1, Ordering::Release);
    }
    pub fn verify(&self, cpu: u32, value: u64) -> bool {
        value == self.get(cpu)
    }
}
