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

//! The randomness the C asks for, from the caller. Each call that may need
//! it lends its source for the length of that call and takes it back. The
//! C does not look at whether a draw succeeded: a failed draw would leave
//! it signing or making keys from zeros. So a failed draw is noted here,
//! and the caller is told the whole call failed.

use core::sync::atomic::{AtomicBool, Ordering};

type Source<'a> = &'a mut dyn FnMut(&mut [u8]) -> bool;

static BUSY: AtomicBool = AtomicBool::new(false);
static FAILED: AtomicBool = AtomicBool::new(false);
type Lent = *mut (dyn FnMut(&mut [u8]) -> bool + 'static);

static mut LENT: Option<Lent> = None;

/// Run `f` with `source` answering the C's randomness; `None` when any
/// draw inside `f` could not be filled. Calls from several threads take
/// turns. A source must not call back into this crate, or it waits on
/// itself; and must not panic, since the panic cannot cross the C and ends
/// the program.
pub(crate) fn with_source<T>(source: Source<'_>, f: impl FnOnce() -> T) -> Option<T> {
    while BUSY.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
        core::hint::spin_loop();
    }
    FAILED.store(false, Ordering::Relaxed);
    // SAFETY: eK@nonos.systems - BUSY makes this the only writer and reader
    // of LENT; the pointer is dropped before `source`'s borrow ends.
    unsafe {
        let p: *mut (dyn FnMut(&mut [u8]) -> bool + '_) = source;
        LENT = Some(core::mem::transmute::<*mut (dyn FnMut(&mut [u8]) -> bool + '_), Lent>(p));
    }
    let out = f();
    // SAFETY: eK@nonos.systems - as above.
    unsafe { LENT = None };
    let failed = FAILED.load(Ordering::Relaxed);
    BUSY.store(false, Ordering::Release);
    (!failed).then_some(out)
}

#[no_mangle]
extern "C" fn nonos_mldsa65_randombytes(buf: *mut u8, n: usize) -> i32 {
    // SAFETY: eK@nonos.systems - only reached from inside `with_source`,
    // which holds BUSY; `buf` is `n` writable bytes the C owns.
    unsafe {
        let out = core::slice::from_raw_parts_mut(buf, n);
        match LENT {
            Some(src) if (*src)(out) => 0,
            _ => {
                FAILED.store(true, Ordering::Relaxed);
                -1
            }
        }
    }
}
