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

use core::sync::atomic::{fence, AtomicU64, Ordering};

/// "NXCLKPG\0", little-endian: the page is the clock page.
pub const MAGIC: u64 = u64::from_le_bytes(*b"NXCLKPG\0");
/// Raised whenever a word changes meaning; a reader built for another
/// version uses the syscall.
pub const VERSION: u64 = 1;

pub const W_MAGIC: usize = 0;
pub const W_VERSION: usize = 1;
/// Odd while the kernel writes; a reader retries until it reads one even
/// value on both sides of its reads.
pub const W_SEQ: usize = 2;
pub const W_FLAGS: usize = 3;
/// Cycle counter ticks per second; 0 until calibrated.
pub const W_HZ: usize = 4;
/// The counter at the clock's zero (`sys::clock::BOOT_TSC`).
pub const W_BOOT_COUNT: usize = 5;
pub const WORDS: usize = 6;

/// User code may read the counter (on x86_64, CR4.TSD is clear).
pub const FLAG_USER_COUNTER: u64 = 1 << 0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Values {
    pub flags: u64,
    pub hz: u64,
    pub boot_count: u64,
}

/*
 * Seqlock writer. Callers serialise (`place::refresh` holds a lock). The
 * odd sequence is visible before any word changes (Release fence after
 * it), and every word is written before the even sequence (Release store).
 */
pub fn publish(words: &[AtomicU64], v: &Values) {
    let seq = words[W_SEQ].load(Ordering::Relaxed);
    words[W_SEQ].store(seq.wrapping_add(1) | 1, Ordering::Relaxed);
    fence(Ordering::Release);
    words[W_FLAGS].store(v.flags, Ordering::Relaxed);
    words[W_HZ].store(v.hz, Ordering::Relaxed);
    words[W_BOOT_COUNT].store(v.boot_count, Ordering::Relaxed);
    words[W_MAGIC].store(MAGIC, Ordering::Relaxed);
    words[W_VERSION].store(VERSION, Ordering::Relaxed);
    words[W_SEQ].store((seq | 1).wrapping_add(1), Ordering::Release);
}

/// Milliseconds since the clock's zero, as `sys::clock::since_boot_ms`
/// computes them from the counter.
pub fn ms_since(count: u64, boot_count: u64, hz: u64) -> u64 {
    if hz == 0 {
        return 0;
    }
    let ms = (count.saturating_sub(boot_count) as u128 * 1000) / hz as u128;
    u64::try_from(ms).unwrap_or(u64::MAX)
}
