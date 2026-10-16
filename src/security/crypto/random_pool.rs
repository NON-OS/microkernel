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

//! The 512-byte pool and its bit count, kept beside the hardware sources.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use spin::Mutex;

use super::random::try_secure_random_u64;

static ENTROPY_POOL: Mutex<[u8; 512]> = Mutex::new([0u8; 512]);
static ENTROPY_BITS: AtomicU32 = AtomicU32::new(0);
static POOL_INDEX: AtomicU32 = AtomicU32::new(0);
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn init() -> Result<(), &'static str> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let mut pool = ENTROPY_POOL.lock();
    for chunk in pool.chunks_mut(8) {
        if let Ok(v) = try_secure_random_u64() {
            let bytes = v.to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len().min(8)]);
        }
    }
    ENTROPY_BITS.store(256, Ordering::SeqCst);
    Ok(())
}

pub fn add_entropy(data: &[u8]) {
    if data.is_empty() {
        return;
    }
    let mut pool = ENTROPY_POOL.lock();
    let mut idx = POOL_INDEX.load(Ordering::Relaxed) as usize;
    for &byte in data {
        pool[idx % 512] ^= byte;
        idx = idx.wrapping_add(1);
    }
    POOL_INDEX.store((idx % 512) as u32, Ordering::Relaxed);
    let estimated_bits = (data.len() * 2).min(256) as u32;
    let current = ENTROPY_BITS.load(Ordering::Relaxed);
    ENTROPY_BITS.store((current + estimated_bits).min(4096), Ordering::Relaxed);
}

pub fn get_entropy_count() -> u32 {
    ENTROPY_BITS.load(Ordering::Relaxed)
}

pub fn add_entropy_count(bits: u32) {
    let current = ENTROPY_BITS.load(Ordering::Relaxed);
    ENTROPY_BITS.store((current + bits).min(4096), Ordering::Relaxed);
}

pub(super) fn consume_entropy(bits: u32) {
    let current = ENTROPY_BITS.load(Ordering::Relaxed);
    ENTROPY_BITS.store(current.saturating_sub(bits), Ordering::Relaxed);
}
