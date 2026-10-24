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

//! The randomness the PQClean C draws. It comes from the seeded RNG only:
//! when that cannot answer, the draw fails and nothing weaker is handed
//! over. The C never looks at whether a draw succeeded, so each failure is
//! also counted here, and the Rust wrappers ask, through `drew_cleanly`,
//! whether a call drew from a failed source, and refuse its result if so.

use core::sync::atomic::{AtomicU64, Ordering};

static FAILURES: AtomicU64 = AtomicU64::new(0);

#[no_mangle]
pub(super) extern "C" fn nonos_randombytes(buf: *mut u8, n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    if buf.is_null() {
        FAILURES.fetch_add(1, Ordering::SeqCst);
        return -1;
    }
    // SAFETY: eK@nonos.systems - the C hands over `n` writable bytes it
    // owns for the length of this call.
    let out = unsafe { core::slice::from_raw_parts_mut(buf, n) };
    match crate::crypto::rng::fill_random_bytes_secure(out) {
        Ok(()) => 0,
        Err(_) => {
            out.fill(0);
            FAILURES.fetch_add(1, Ordering::SeqCst);
            -1
        }
    }
}

/// Run `call`, then say whether it drew only from a working source. A
/// failure on another CPU during the call counts against this one too:
/// that refuses a call that might have been sound, and never accepts one
/// that was not.
pub(crate) fn drew_cleanly<T>(call: impl FnOnce() -> T) -> (T, bool) {
    let before = FAILURES.load(Ordering::SeqCst);
    let out = call();
    (out, FAILURES.load(Ordering::SeqCst) == before)
}
