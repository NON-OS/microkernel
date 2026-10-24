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

//! What the boot CPU decided about pointer authentication, read by every
//! other CPU before it starts: either all of them sign with one kernel key
//! or none does, because a thread's signed frames move with it between CPUs.

use core::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};

static ACTIVE: AtomicBool = AtomicBool::new(false);
static STATUS: AtomicU8 = AtomicU8::new(0);
static KERNEL_KEY: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];
static USER_SECRET: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];

/// The boot CPU, once, before any other CPU runs.
pub(super) fn publish(kernel: [u64; 2], user_secret: &[u8; 32], status: u8) {
    for (word, value) in KERNEL_KEY.iter().zip(kernel) {
        word.store(value, Ordering::Relaxed);
    }
    for (word, bytes) in USER_SECRET.iter().zip(user_secret.chunks_exact(8)) {
        let mut le = [0u8; 8];
        le.copy_from_slice(bytes);
        word.store(u64::from_le_bytes(le), Ordering::Relaxed);
    }
    STATUS.store(status, Ordering::Relaxed);
    ACTIVE.store(true, Ordering::Release);
}

pub(super) fn record(status: u8) {
    STATUS.store(status, Ordering::Release);
}

pub fn active() -> bool {
    ACTIVE.load(Ordering::Acquire)
}

pub(super) fn status() -> u8 {
    STATUS.load(Ordering::Acquire)
}

pub(super) fn kernel_key() -> [u64; 2] {
    [KERNEL_KEY[0].load(Ordering::Relaxed), KERNEL_KEY[1].load(Ordering::Relaxed)]
}

pub(super) fn user_secret() -> [u8; 32] {
    let mut out = [0u8; 32];
    for (bytes, word) in out.chunks_exact_mut(8).zip(USER_SECRET.iter()) {
        bytes.copy_from_slice(&word.load(Ordering::Relaxed).to_le_bytes());
    }
    out
}
