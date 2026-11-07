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

//! Source-over on premultiplied pixels (`scalar::over`), four ways round:
//! a row laid over the destination, under it, over one colour, or the
//! destination itself flattened onto one colour.

use crate::level::{level, Level};
use crate::scalar::{load, over};

/// The kernels take the mode as a constant, so the choice is made once.
pub const SRC_OVER: u8 = 0;
pub const SRC_UNDER: u8 = 1;
pub const OVER_COLOUR: u8 = 2;
pub const FLATTEN: u8 = 3;

/// `dst = src over dst`.
pub fn over_row(dst: &mut [u8], src: &[u8]) {
    let n = dst.len().min(src.len()) / 4 * 4;
    // SAFETY: both hold `n` bytes.
    unsafe { run::<SRC_OVER>(dst.as_mut_ptr(), src.as_ptr(), n, 0) }
}

/// `dst = dst over src`: a layer slid beneath what is there.
pub fn under_row(dst: &mut [u8], src: &[u8]) {
    let n = dst.len().min(src.len()) / 4 * 4;
    // SAFETY: both hold `n` bytes.
    unsafe { run::<SRC_UNDER>(dst.as_mut_ptr(), src.as_ptr(), n, 0) }
}

/// `dst = src over colour`: a translucent row flattened onto a ground.
pub fn over_colour(dst: &mut [u8], src: &[u8], colour: u32) {
    let n = dst.len().min(src.len()) / 4 * 4;
    // SAFETY: both hold `n` bytes.
    unsafe { run::<OVER_COLOUR>(dst.as_mut_ptr(), src.as_ptr(), n, colour) }
}

/// `row = row over colour`, in place.
pub fn flatten(row: &mut [u8], colour: u32) {
    let n = row.len() / 4 * 4;
    let p = row.as_mut_ptr();
    // SAFETY: `row` holds `n` bytes; the flatten mode reads only `dst`.
    unsafe { run::<FLATTEN>(p, p, n, colour) }
}

/// # Safety
/// `dst` holds `n` bytes, and `src` does for the modes that read it.
unsafe fn run<const M: u8>(dst: *mut u8, src: *const u8, n: usize, colour: u32) {
    let done = match level() {
        #[cfg(target_arch = "x86_64")]
        Level::Avx2 => {
            let k = n / 32 * 32;
            crate::avx2::blend::<M>(dst, src, k, colour);
            k
        }
        #[cfg(target_arch = "x86_64")]
        Level::Sse2 => {
            let k = n / 16 * 16;
            crate::sse2::blend::<M>(dst, src, k, colour);
            k
        }
        _ => 0,
    };
    scalar::<M>(dst.add(done), src.add(done), n - done, colour);
}

/// The plain path, and the tail of the vector ones.
///
/// # Safety
/// As `run`.
pub unsafe fn scalar<const M: u8>(dst: *mut u8, src: *const u8, n: usize, colour: u32) {
    let mut i = 0;
    while i + 4 <= n {
        let d = load(core::slice::from_raw_parts(dst.add(i), 4));
        let s = || load(core::slice::from_raw_parts(src.add(i), 4));
        let out = match M {
            SRC_OVER => over(s(), d),
            SRC_UNDER => over(d, s()),
            OVER_COLOUR => over(s(), colour),
            _ => over(d, colour),
        };
        core::ptr::copy_nonoverlapping(out.to_le_bytes().as_ptr(), dst.add(i), 4);
        i += 4;
    }
}
