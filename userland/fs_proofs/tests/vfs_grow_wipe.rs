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
//! A written file's room past its length given back when its writer closes
//! A file's bytes are cleared from every heap block the vfs store gives
//! back (`capsule_vfs/src/store/fdtable/budget.rs` grow and trim): as a
//! file grows its bytes move to a larger block and the old one is freed,
//! and as its writer closes it they move to one of their length. A pass
//! through allocator looks at each block freed while the store works and
//! counts those that still hold a run of the file's bytes. Each test writes
//! a byte of its own and runs alone, so no block another test freed with
//! its bytes in it, then handed out again, is counted for this one.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering::SeqCst};
use std::sync::{Mutex, MutexGuard};

use fs_proofs::store::Store;

/// How long a run of the test's byte marks a block.
const RUN: usize = 64;

struct Looking;

static ARMED: AtomicBool = AtomicBool::new(false);
static MARK: AtomicU8 = AtomicU8::new(0);
static MARKED: AtomicUsize = AtomicUsize::new(0);
static FREED: AtomicUsize = AtomicUsize::new(0);
static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

/// Whether the `len` bytes at `p` hold RUN bytes of the test's byte in a row.
fn holds_run(p: *const u8, len: usize) -> bool {
    // SAFETY: `p` is a live block of `len` bytes, read before it is freed.
    let bytes = unsafe { core::slice::from_raw_parts(p, len) };
    let mark = MARK.load(SeqCst);
    bytes.windows(RUN).any(|w| w.iter().all(|&b| b == mark))
}

unsafe impl GlobalAlloc for Looking {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        if ARMED.load(SeqCst) {
            FREED.fetch_add(1, SeqCst);
            if holds_run(p, layout.size()) {
                MARKED.fetch_add(1, SeqCst);
            }
        }
        unsafe { System.dealloc(p, layout) }
    }
}

#[global_allocator]
static LOOKING: Looking = Looking;

/// The test that writes `mark` runs alone until the guard goes.
fn alone(mark: u8) -> MutexGuard<'static, ()> {
    let one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    MARK.store(mark, SeqCst);
    one
}

/// Blocks freed during `f`, and how many of them held a run of the mark.
fn freed_during(f: impl FnOnce()) -> (usize, usize) {
    FREED.store(0, SeqCst);
    MARKED.store(0, SeqCst);
    ARMED.store(true, SeqCst);
    f();
    ARMED.store(false, SeqCst);
    (FREED.load(SeqCst), MARKED.load(SeqCst))
}

const A: u32 = 40;
const PIECE: usize = 4096;

#[test]
fn a_growing_file_leaves_none_of_its_bytes_in_the_blocks_it_moves_out_of() {
    let _alone = alone(0x5A);
    let mut s = Store::new();
    let fd = s.open("/grows", A, true, false, false, true).expect("opens");
    let piece = vec![0x5A; PIECE];
    let (freed, marked) = freed_during(|| {
        for _ in 0..64 {
            assert_eq!(s.write(fd, A, &piece), Ok(PIECE as u32));
        }
    });
    assert!(freed >= 5, "64 pieces grew the file by doubling, {freed} blocks freed");
    assert_eq!(marked, 0, "a block the file moved out of still held its bytes");
}

#[test]
fn a_close_that_trims_and_an_unlink_leave_none_of_its_bytes() {
    let _alone = alone(0xA7);
    let mut s = Store::new();
    let fd = s.open("/trims", A, true, false, false, true).expect("opens");
    let piece = vec![0xA7; PIECE];
    for _ in 0..40 {
        assert_eq!(s.write(fd, A, &piece), Ok(PIECE as u32));
    }
    let held = s.held_except(None);
    let (freed, marked) = freed_during(|| s.close(fd, A).expect("closes"));
    assert!(s.held_except(None) < held, "the close gave the slack back");
    assert!(freed >= 1);
    assert_eq!(marked, 0, "the block the close moved out of still held its bytes");
    let (_, marked) = freed_during(|| s.unlink("/trims").expect("unlinks"));
    assert_eq!(marked, 0, "an unlinked file's block still held its bytes");
}
