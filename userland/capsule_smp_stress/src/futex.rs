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

//! The futex calls, raw: libc carries the wait only inside `mk_idle_ms`.

use core::sync::atomic::AtomicU32;
use nonos_libc::mk_syscall_raw as call_raw;

/// The kernel's syscall tag, as libc's numbers table builds it.
const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const FUTEX_WAIT: i64 = tag4(b"MFTW");
const FUTEX_WAKE: i64 = tag4(b"MFTK");

/// Sleep while `word` holds `expected`, for at most `timeout_ms`.
pub fn wait(word: &AtomicU32, expected: u32, timeout_ms: u64) -> i64 {
    call_raw(FUTEX_WAIT, [word.as_ptr() as u64, u64::from(expected), timeout_ms, 0, 0, 0])
}

/// Wake up to `count` threads sleeping on `word`.
pub fn wake(word: &AtomicU32, count: u64) -> i64 {
    call_raw(FUTEX_WAKE, [word.as_ptr() as u64, count, 0, 0, 0, 0])
}
