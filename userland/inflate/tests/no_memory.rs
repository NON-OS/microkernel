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

//! When the allocator cannot give the output room, a decode stops and says
//! so, never aborts: the install role inflated a 29.5 MB package member in
//! a heap that could not hold one more block that size, and the process
//! ended (code 134) with no reason given. This binary's allocator refuses
//! any block above LIMIT, so the refusal can be made on purpose.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use nonos_inflate::{gunzip_partial, members_checked, End, Refused};

static LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);

struct Refusing;

unsafe impl GlobalAlloc for Refusing {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if l.size() > LIMIT.load(Ordering::Relaxed) {
            return core::ptr::null_mut();
        }
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
}

#[global_allocator]
static A: Refusing = Refusing;

const BIG: &[u8] = include_bytes!("vectors/big.gz");
const APK: &[u8] = include_bytes!("vectors/apk_shape.gz");
const MIB: usize = 1 << 20;

/* One test: LIMIT is the whole binary's, and tests run on threads. */
#[test]
fn a_refused_allocation_is_a_reason_not_an_abort() {
    assert_eq!(gunzip_partial(BIG, 4 * MIB).end, End::Complete, "the vector opens with room");
    LIMIT.store(256 << 10, Ordering::Relaxed);
    let r = gunzip_partial(BIG, 4 * MIB);
    assert_eq!(r.end, End::NoMemory);
    assert!(r.short.is_some_and(|n| n > 256 << 10), "it says how much it asked: {:?}", r.short);
    match members_checked(APK, 4 * MIB) {
        Err(Refused::NoMemory(n)) => assert!(n > 256 << 10, "asked {n}"),
        other => panic!("not a NoMemory refusal: {:?}", other.map(|m| m.len())),
    }
    LIMIT.store(usize::MAX, Ordering::Relaxed);
    assert_eq!(members_checked(APK, 4 * MIB).map(|m| m.len()), Ok(3), "and opens again with room");
}
