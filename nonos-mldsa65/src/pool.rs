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

//! The memory the C asks for. ML-DSA-65 allocates only SHAKE states: 208
//! bytes for an incremental one, 200 for a one-shot one. It holds at most
//! one at a time, and only one call is in the C at once (`rng`), so four
//! fixed 256-byte slots answer every request and no allocator is needed. A size
//! over a slot, or a fifth live request, gets nothing, and the C then
//! calls exit, which ends the program rather than continue without state.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

const SLOT: usize = 256;
const SLOTS: usize = 4;

#[repr(C, align(16))]
struct Slots(UnsafeCell<[[u8; SLOT]; SLOTS]>);

// SAFETY: eK@nonos.systems - each slot is handed to one holder at a time,
// guarded by its own flag in TAKEN.
unsafe impl Sync for Slots {}

static MEM: Slots = Slots(UnsafeCell::new([[0; SLOT]; SLOTS]));
static TAKEN: [AtomicBool; SLOTS] = [const { AtomicBool::new(false) }; SLOTS];

fn slot(i: usize) -> *mut u8 {
    // SAFETY: eK@nonos.systems - i < SLOTS, so the pointer is in MEM.
    unsafe { (*MEM.0.get())[i].as_mut_ptr() }
}

#[no_mangle]
extern "C" fn nonos_mldsa65_malloc(size: usize) -> *mut u8 {
    if size > SLOT {
        return core::ptr::null_mut();
    }
    for (i, t) in TAKEN.iter().enumerate() {
        if !t.swap(true, Ordering::Acquire) {
            return slot(i);
        }
    }
    core::ptr::null_mut()
}

#[no_mangle]
extern "C" fn nonos_mldsa65_free(p: *mut u8) {
    if let Some(i) = (0..SLOTS).find(|i| slot(*i) == p) {
        // SAFETY: eK@nonos.systems - the slot belongs to the caller until
        // its flag is cleared below; wiped so the next holder sees zeros.
        unsafe { core::ptr::write_bytes(p, 0, SLOT) };
        TAKEN[i].store(false, Ordering::Release);
    }
}

#[no_mangle]
extern "C" fn nonos_mldsa65_exit(_status: i32) -> ! {
    panic!("ML-DSA-65 could not get the memory it needs");
}
