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

//! The per-boot address key is wiped where it lies when its holder goes:
//! the vault's box reads zero as it is freed. This binary's allocator looks
//! at each block of the key's size freed while the test watches.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use nonos_wifi_core::address::{Addresses, KEY_LEN};

thread_local! {
    static WATCHING: Cell<bool> = const { Cell::new(false) };
    static FREED_ZERO: Cell<Option<bool>> = const { Cell::new(None) };
}

struct Watch;

// SAFETY: every call goes to the system allocator unchanged; dealloc only
// reads the block it is handed before passing it on.
unsafe impl GlobalAlloc for Watch {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller's contract for `alloc`, passed on.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let watching = WATCHING.try_with(Cell::get).unwrap_or(false);
        if watching && layout.size() == KEY_LEN {
            // SAFETY: `ptr` is a live block of `layout.size()` bytes until the
            // system allocator frees it below.
            let block = unsafe { core::slice::from_raw_parts(ptr, layout.size()) };
            let zero = block.iter().all(|&b| b == 0);
            let _ = FREED_ZERO.try_with(|f| f.set(Some(zero)));
        }
        // SAFETY: the caller's contract for `dealloc`, passed on.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Watch = Watch;

#[test]
fn the_address_key_is_gone_after_it_is_dropped() {
    let a = Addresses::new(|k| {
        k.fill(0x5A);
        true
    })
    .expect("a key");
    assert_ne!(a.join(b"home", 1), [0; 6]);
    WATCHING.with(|w| w.set(true));
    drop(a);
    WATCHING.with(|w| w.set(false));
    assert_eq!(FREED_ZERO.with(Cell::get), Some(true), "the key read zero as it was freed");
}
