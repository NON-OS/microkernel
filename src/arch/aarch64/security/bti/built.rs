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

//! Whether this kernel was compiled with the branch protection the security
//! code is about to turn on, read from its own instructions rather than
//! assumed: a build that lost its rustflags would otherwise run with guarded
//! text and no landing pads, and fault on its first indirect call.
//!
//! `-Z branch-protection=bti` starts every function that can be reached
//! indirectly with BTI c, or with PACIASP, which is also a landing pad, when
//! the function signs its return address. `pac-ret` starts every function
//! that saves its link register with PACIASP. A leaf that is address-taken
//! shows the first, a non-leaf the second.

use super::pad::{BTI_C, BTI_JC, PACIASP};

#[inline(never)]
extern "C" fn leaf_probe() {}

#[inline(never)]
extern "C" fn calling_probe(f: extern "C" fn()) {
    f();
    f();
}

fn first_two(f: usize) -> [u32; 2] {
    // SAFETY: `f` is the address of a function in this image's text, which
    // is mapped readable, and every function is at least two instructions
    // or ends with RET followed by the next function's first.
    unsafe {
        [core::ptr::read_volatile(f as *const u32), core::ptr::read_volatile((f + 4) as *const u32)]
    }
}

/// Address-taken functions start with a landing pad.
pub fn built_with_bti() -> bool {
    let probe: extern "C" fn() = leaf_probe;
    matches!(first_two(probe as usize)[0], BTI_C | BTI_JC)
}

/// Functions that save the link register sign it first.
pub fn built_with_pac_ret() -> bool {
    let probe: extern "C" fn(extern "C" fn()) = calling_probe;
    first_two(probe as usize).contains(&PACIASP)
}
