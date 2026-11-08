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

//! Every path a row can take on this host, so each is held to the same
//! answer whatever `level` picks here.

use crate::blend;

pub type Kernel = fn(&mut [u8], &[u8], u32, u8);

/// The bytes a mode works on: the flatten reads no source.
fn span(dst: &[u8], src: &[u8], mode: u8, block: usize) -> usize {
    let n = if mode == blend::FLATTEN { dst.len() } else { dst.len().min(src.len()) };
    n / block * block
}

fn scalar(dst: &mut [u8], src: &[u8], c: u32, mode: u8) {
    let n = span(dst, src, mode, 4);
    let (d, s) = (dst.as_mut_ptr(), src.as_ptr());
    // SAFETY: both hold `n` bytes.
    unsafe {
        match mode {
            blend::SRC_OVER => blend::scalar::<{ blend::SRC_OVER }>(d, s, n, c),
            blend::SRC_UNDER => blend::scalar::<{ blend::SRC_UNDER }>(d, s, n, c),
            blend::OVER_COLOUR => blend::scalar::<{ blend::OVER_COLOUR }>(d, s, n, c),
            _ => blend::scalar::<{ blend::FLATTEN }>(d, d, n, c),
        }
    }
}

#[cfg_attr(not(target_arch = "x86_64"), allow(unused_macros))]
macro_rules! vector {
    ($name:ident, $m:ident, $block:expr) => {
        fn $name(dst: &mut [u8], src: &[u8], c: u32, mode: u8) {
            let n = span(dst, src, mode, $block);
            let (d, s) = (dst.as_mut_ptr(), src.as_ptr());
            // SAFETY: the host runs the instructions (checked by `kernels`);
            // both hold `n` bytes.
            unsafe {
                match mode {
                    blend::SRC_OVER => crate::$m::blend::<{ blend::SRC_OVER }>(d, s, n, c),
                    blend::SRC_UNDER => crate::$m::blend::<{ blend::SRC_UNDER }>(d, s, n, c),
                    blend::OVER_COLOUR => crate::$m::blend::<{ blend::OVER_COLOUR }>(d, s, n, c),
                    _ => crate::$m::blend::<{ blend::FLATTEN }>(d, d, n, c),
                }
            }
            scalar(&mut dst[n..], &src[n.min(src.len())..], c, mode);
        }
    };
}

#[cfg(target_arch = "x86_64")]
vector!(sse2, sse2, 16);
#[cfg(target_arch = "x86_64")]
vector!(avx2, avx2, 32);

/// The plain path and each vector path this host runs, by name.
pub fn kernels() -> Vec<(&'static str, Kernel)> {
    #[cfg_attr(not(target_arch = "x86_64"), allow(unused_mut))]
    let mut k: Vec<(&'static str, Kernel)> = vec![("scalar", scalar)];
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("sse2") {
            k.push(("sse2", sse2));
        }
        if std::is_x86_feature_detected!("avx2") {
            k.push(("avx2", avx2));
        }
    }
    k
}
