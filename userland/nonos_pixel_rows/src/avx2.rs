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

//! The AVX2 path: eight pixels at a time, the same lane arithmetic as
//! SSE2 in each 128-bit half.

use core::arch::x86_64::*;

use crate::blend::{OVER_COLOUR, SRC_OVER, SRC_UNDER};

/// Alpha set on whole blocks of eight pixels; returns the bytes done.
///
/// # Safety
/// The processor runs AVX2 and the kernel saves its state.
#[target_feature(enable = "avx2")]
pub unsafe fn opaque(row: &mut [u8]) -> usize {
    let n = row.len() / 32 * 32;
    let alpha = _mm256_set1_epi32(0xFF00_0000u32 as i32);
    let mut i = 0;
    while i < n {
        let p = row.as_mut_ptr().add(i) as *mut __m256i;
        _mm256_storeu_si256(p, _mm256_or_si256(_mm256_loadu_si256(p), alpha));
        i += 32;
    }
    n
}

/// `scalar::over` on the first `n` bytes, `n` a whole number of blocks.
/// `src` is read only by the modes that use it, and may be `dst`.
///
/// # Safety
/// The processor runs AVX2 and the kernel saves its state; `dst`, and `src` for the
/// modes that read it, hold `n` bytes.
#[target_feature(enable = "avx2")]
pub unsafe fn blend<const M: u8>(dst: *mut u8, src: *const u8, n: usize, colour: u32) {
    let c = _mm256_set1_epi32(colour as i32);
    let alpha = _mm256_set1_epi32(0xFF00_0000u32 as i32);
    let mut i = 0;
    while i < n {
        let dp = dst.add(i) as *mut __m256i;
        let d = _mm256_loadu_si256(dp);
        let (u, l) = match M {
            SRC_OVER => (_mm256_loadu_si256(src.add(i) as *const __m256i), d),
            SRC_UNDER => (d, _mm256_loadu_si256(src.add(i) as *const __m256i)),
            OVER_COLOUR => (_mm256_loadu_si256(src.add(i) as *const __m256i), c),
            _ => (d, c),
        };
        // All eight upper pixels opaque: over gives the upper ones exactly
        // (255 - 255 keeps none of the lower), so they are stored as they
        // are, and where the upper ones are the destination's, left there.
        let a = _mm256_and_si256(u, alpha);
        if _mm256_movemask_epi8(_mm256_cmpeq_epi32(a, alpha)) == -1 {
            if M == SRC_OVER || M == OVER_COLOUR {
                _mm256_storeu_si256(dp, u);
            }
        } else {
            _mm256_storeu_si256(dp, over8(u, l));
        }
        i += 32;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn over8(u: __m256i, l: __m256i) -> __m256i {
    let z = _mm256_setzero_si256();
    let lo = over4(_mm256_unpacklo_epi8(u, z), _mm256_unpacklo_epi8(l, z));
    let hi = over4(_mm256_unpackhi_epi8(u, z), _mm256_unpackhi_epi8(l, z));
    _mm256_packus_epi16(lo, hi)
}

/// Four pixels as sixteen 16-bit channels: `u + div255(l * (255 - ua) + 127)`.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn over4(u: __m256i, l: __m256i) -> __m256i {
    let a = _mm256_shufflehi_epi16::<0xFF>(_mm256_shufflelo_epi16::<0xFF>(u));
    let keep = _mm256_sub_epi16(_mm256_set1_epi16(255), a);
    let v = _mm256_add_epi16(_mm256_mullo_epi16(l, keep), _mm256_set1_epi16(127));
    let q = _mm256_add_epi16(_mm256_add_epi16(v, _mm256_srli_epi16::<8>(v)), _mm256_set1_epi16(1));
    _mm256_add_epi16(u, _mm256_srli_epi16::<8>(q))
}

/// The compositor's layer rule (`composite::scalar`) on the first `n`
/// bytes, `n` a whole number of blocks. A block all opaque is stored as it
/// is and one all clear is skipped, as the plain path's runs are.
///
/// # Safety
/// The processor runs AVX2 and the kernel saves its state; both hold `n` bytes and
/// do not overlap.
#[target_feature(enable = "avx2")]
pub unsafe fn composite(dst: *mut u8, src: *const u8, n: usize) {
    let alpha = _mm256_set1_epi32(0xFF00_0000u32 as i32);
    let mut i = 0;
    while i < n {
        let s = _mm256_loadu_si256(src.add(i) as *const __m256i);
        let dp = dst.add(i) as *mut __m256i;
        let a = _mm256_and_si256(s, alpha);
        let opaque = _mm256_movemask_epi8(_mm256_cmpeq_epi32(a, alpha));
        let clear = _mm256_cmpeq_epi32(a, _mm256_setzero_si256());
        if opaque == -1 {
            _mm256_storeu_si256(dp, s);
        } else if _mm256_movemask_epi8(clear) != -1 {
            let d = _mm256_loadu_si256(dp);
            let lo = mix2(
                _mm256_unpacklo_epi8(s, _mm256_setzero_si256()),
                _mm256_unpacklo_epi8(d, _mm256_setzero_si256()),
            );
            let hi = mix2(
                _mm256_unpackhi_epi8(s, _mm256_setzero_si256()),
                _mm256_unpackhi_epi8(d, _mm256_setzero_si256()),
            );
            let mixed = _mm256_or_si256(_mm256_packus_epi16(lo, hi), alpha);
            // A clear pixel keeps what was there, alpha and all.
            _mm256_storeu_si256(
                dp,
                _mm256_or_si256(_mm256_and_si256(clear, d), _mm256_andnot_si256(clear, mixed)),
            );
        }
        i += 32;
    }
}

/// Four pixels as 16-bit channels: `(x + 1 + (x >> 8)) >> 8`, `x = s * a + d * (255 - a)`.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mix2(s: __m256i, d: __m256i) -> __m256i {
    let a = _mm256_shufflehi_epi16::<0xFF>(_mm256_shufflelo_epi16::<0xFF>(s));
    let ia = _mm256_sub_epi16(_mm256_set1_epi16(255), a);
    let x = _mm256_add_epi16(_mm256_mullo_epi16(s, a), _mm256_mullo_epi16(d, ia));
    let x = _mm256_add_epi16(_mm256_add_epi16(x, _mm256_srli_epi16::<8>(x)), _mm256_set1_epi16(1));
    _mm256_srli_epi16::<8>(x)
}
