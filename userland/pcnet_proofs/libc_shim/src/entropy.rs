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

//! `crypto_random`, switchable per thread so a test can take entropy away.

use std::cell::Cell;

thread_local! {
    static ENTROPY: Cell<bool> = const { Cell::new(true) };
}

pub fn entropy(available: bool) {
    ENTROPY.with(|e| e.set(available));
}

/// A fixed pattern when entropy is on, so a test can recognise the drawn
/// address; a refusal when it is off.
pub fn crypto_random(ptr: *mut u8, len: usize) -> i64 {
    if !ENTROPY.with(|e| e.get()) {
        return -1;
    }
    for i in 0..len {
        // SAFETY: the caller hands `len` writable bytes, as the real call requires.
        unsafe { *ptr.add(i) = 0xC3 ^ (i as u8) };
    }
    len as i64
}
