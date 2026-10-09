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

//! Reading this terminal's registry entry from the kernel, once.
//!
//! The entry is fixed for the life of the process (the kernel records it when
//! it admits the capsule), and the fresh-tab splash draws it every frame, so
//! the registry is read the first time it is asked for and the line kept.

use alloc::vec;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use nonos_libc::{mk_attest_entries, mk_getpid, ATTEST_ENTRY_LEN};

use super::own::{own_signer, ENTRY_LEN};

/// Room for the whole registry, as `receipt` sizes it.
const MAX_ENTRIES: usize = 256;
const CAP: usize = 96;

// The entry layout `own` parses is the one the kernel writes.
const _: () = assert!(ATTEST_ENTRY_LEN == ENTRY_LEN);

struct Kept {
    buf: UnsafeCell<[u8; CAP]>,
    len: AtomicUsize,
    asked: AtomicBool,
}

/*
 * Safety: a capsule is single threaded and the cell is written exactly once,
 * by the first `own_line`, before `len` publishes the length any reader uses.
 */
unsafe impl Sync for Kept {}

static KEPT: Kept = Kept {
    buf: UnsafeCell::new([0u8; CAP]),
    len: AtomicUsize::new(0),
    asked: AtomicBool::new(false),
};

/// Who signed this terminal and its measurement, or why that is unknown.
pub fn own_line() -> &'static [u8] {
    if !KEPT.asked.swap(true, Ordering::Relaxed) {
        let mut regs = vec![0u8; MAX_ENTRIES * ENTRY_LEN];
        let rc = mk_attest_entries(&mut regs);
        let read = if rc < 0 { Err(rc) } else { Ok(&regs[..(rc as usize).min(regs.len())]) };
        let line = own_signer(read, mk_getpid());
        let n = line.len().min(CAP);
        let slot: &mut [u8; CAP] = unsafe { &mut *KEPT.buf.get() };
        slot[..n].copy_from_slice(&line[..n]);
        KEPT.len.store(n, Ordering::Release);
    }
    let n = KEPT.len.load(Ordering::Acquire);
    let slot: &'static [u8; CAP] = unsafe { &*KEPT.buf.get() };
    &slot[..n]
}
