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

//! Wiping and comparing secrets: volatile stores the optimiser keeps, and a
//! comparison whose time does not depend on where two tags differ.

/// Zero every byte with a store the compiler may not drop.
pub fn secure_zero(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        /* SAFETY: `b` is a valid, aligned and exclusive reference. */
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

/// Whether two byte strings are equal, in time that depends on their
/// lengths only.
pub fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    /* Read through a volatile load so the fold is not cut short. */
    // SAFETY: `diff` is a local, valid and aligned.
    let seen = unsafe { core::ptr::read_volatile(&diff) };
    seen == 0
}
