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

//! The monotonic clock read from the kernel's clock page, with no syscall.
//! The layout is the kernel's (`src/sys/clock/page/layout.rs`), held equal
//! to this reader in `launch_proofs` (`clock_tests.rs`). Anything unexpected on the page
//! (another magic or version, the counter not readable from user code, no
//! rate, a write that never settles) answers None, and the caller uses
//! `MkTimeMonotonic`.

use core::sync::atomic::{fence, AtomicU64, Ordering};

pub const MAGIC: u64 = u64::from_le_bytes(*b"NXCLKPG\0");
pub const VERSION: u64 = 1;
pub const W_MAGIC: usize = 0;
pub const W_VERSION: usize = 1;
pub const W_SEQ: usize = 2;
pub const W_FLAGS: usize = 3;
pub const W_HZ: usize = 4;
pub const W_BOOT_COUNT: usize = 5;
pub const WORDS: usize = 6;
pub const FLAG_USER_COUNTER: u64 = 1 << 0;

/// Reads before giving up on a page the kernel is writing.
pub const TRIES: usize = 4;

/// Milliseconds since the clock's zero at counter value `count`, or None.
pub fn read(words: &[AtomicU64], count: impl Fn() -> u64) -> Option<u64> {
    if words.len() < WORDS {
        return None;
    }
    for _ in 0..TRIES {
        let before = words[W_SEQ].load(Ordering::Acquire);
        if before & 1 != 0 {
            core::hint::spin_loop();
            continue;
        }
        let magic = words[W_MAGIC].load(Ordering::Relaxed);
        let version = words[W_VERSION].load(Ordering::Relaxed);
        let flags = words[W_FLAGS].load(Ordering::Relaxed);
        let hz = words[W_HZ].load(Ordering::Relaxed);
        let boot = words[W_BOOT_COUNT].load(Ordering::Relaxed);
        let now = count();
        fence(Ordering::Acquire);
        if words[W_SEQ].load(Ordering::Relaxed) != before {
            continue;
        }
        if magic != MAGIC || version != VERSION || flags & FLAG_USER_COUNTER == 0 || hz == 0 {
            return None;
        }
        let ms = (now.saturating_sub(boot) as u128 * 1000) / hz as u128;
        return Some(u64::try_from(ms).unwrap_or(u64::MAX));
    }
    None
}
