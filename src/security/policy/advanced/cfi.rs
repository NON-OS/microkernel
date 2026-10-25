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

//! Control-flow integrity and address-space layout randomisation.

extern crate alloc;

use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::RwLock;

pub struct CFI {
    pub(super) enabled: AtomicBool,
    targets: RwLock<BTreeMap<u64, u32>>,
}
impl CFI {
    pub fn new() -> Self {
        Self { enabled: AtomicBool::new(false), targets: RwLock::new(BTreeMap::new()) }
    }
    pub fn add_target(&self, addr: u64, sig: u32) {
        self.targets.write().insert(addr, sig);
    }
    pub fn validate(&self, addr: u64) -> bool {
        if !self.enabled.load(Ordering::Relaxed) {
            return true;
        }
        let lock = self.targets.read();
        lock.contains_key(&addr)
    }
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
    }
}

pub struct ASLR {
    entropy_bits: u32,
    pub(super) enabled: AtomicBool,
}
impl ASLR {
    pub fn new(entropy: u32) -> Self {
        Self { entropy_bits: entropy, enabled: AtomicBool::new(true) }
    }
    /// `None` when no randomness answers; a slide is never made without it.
    pub fn randomize(&self, base: u64) -> Option<u64> {
        if !self.enabled.load(Ordering::Relaxed) {
            return Some(base);
        }
        let slide = crate::crypto::secure_random_u64().ok()? & ((1 << self.entropy_bits) - 1);
        Some((base + (slide << 12)) & 0xFFFF_FFFF_FFFF_F000)
    }
}
