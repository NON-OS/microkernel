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

//! The SSE2 path: four pixels at a time. SSE2 is in every x86-64.

use core::arch::x86_64::*;

use crate::blend::{OVER_COLOUR, SRC_OVER, SRC_UNDER};

/// Alpha set on whole blocks of four pixels; returns the bytes done.
///
/// # Safety
/// The processor runs SSE2.
#[target_feature(enable = "sse2")]
pub unsafe fn opaque(row: &mut [u8]) -> usize {
    let n = row.len() / 16 * 16;
    let alpha = _mm_set1_epi32(0xFF00_0000u32 as i32);
    let mut i = 0;
    while i < n {
        let p = row.as_mut_ptr().add(i) as *mut __m128i;
        _mm_storeu_si128(p, _mm_or_si128(_mm_loadu_si128(p), alpha));
        i += 16;
    }
    n
}

/// `scalar::over` on the first `n` bytes, `n` a whole number of blocks.
/// `src` is read only by the modes that use it, and may be `dst`.
///
/// # Safety
/// The processor runs SSE2; `dst`, and `src` for the
/// modes that read it, hold `n` bytes.
#[target_feature(enable = "sse2")]
pub unsafe fn blend<const M: u8>(dst: *mut u8, src: *const u8, n: usize, colour: u32) {
    let c = _mm_set1_epi32(colour as i32);
    let alpha = _mm_set1_epi32(0xFF00_0000u32 as i32);
    let mut i = 0;
    while i < n {
        let dp = dst.add(i) as *mut __m128i;
        let d = _mm_loadu_si128(dp);
        let (u, l) = match M {
            SRC_OVER => (_mm_loadu_si128(src.add(i) as *const __m128i), d),
            SRC_UNDER => (d, _mm_loadu_si128(src.add(i) as *const __m128i)),
            OVER_COLOUR => (_mm_loadu_si128(src.add(i) as *const __m128i), c),
            _ => (d, c),
        };
        // All four upper pixels opaque: over gives the upper ones exactly
        // (255 - 255 keeps none of the lower), so they are stored as they
        // are, and where the upper ones are the destination's, left there.
        let a = _mm_and_si128(u, alpha);
        if _mm_movemask_epi8(_mm_cmpeq_epi32(a, alpha)) == 0xFFFF {
            if M == SRC_OVER || M == OVER_COLOUR {
                _mm_storeu_si128(dp, u);
            }
        } else {
            _mm_storeu_si128(dp, over4(u, l));
        }
        i += 16;
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn over4(u: __m128i, l: __m128i) -> __m128i {
    let z = _mm_setzero_si128();
    let lo = over2(_mm_unpacklo_epi8(u, z), _mm_unpacklo_epi8(l, z));
    let hi = over2(_mm_unpackhi_epi8(u, z), _mm_unpackhi_epi8(l, z));
    _mm_packus_epi16(lo, hi)
}

/// Two pixels as eight 16-bit channels: `u + div255(l * (255 - ua) + 127)`.
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn over2(u: __m128i, l: __m128i) -> __m128i {
    let a = _mm_shufflehi_epi16::<0xFF>(_mm_shufflelo_epi16::<0xFF>(u));
    let keep = _mm_sub_epi16(_mm_set1_epi16(255), a);
    let v = _mm_add_epi16(_mm_mullo_epi16(l, keep), _mm_set1_epi16(127));
    let q = _mm_add_epi16(_mm_add_epi16(v, _mm_srli_epi16::<8>(v)), _mm_set1_epi16(1));
    _mm_add_epi16(u, _mm_srli_epi16::<8>(q))
}

/// The compositor's layer rule (`composite::scalar`) on the first `n`
/// bytes, `n` a whole number of blocks. A block all opaque is stored as it
/// is and one all clear is skipped, as the plain path's runs are.
///
/// # Safety
/// The processor runs SSE2; both hold `n` bytes and
/// do not overlap.
#[target_feature(enable = "sse2")]
pub unsafe fn composite(dst: *mut u8, src: *const u8, n: usize) {
    let alpha = _mm_set1_epi32(0xFF00_0000u32 as i32);
    let mut i = 0;
    while i < n {
        let s = _mm_loadu_si128(src.add(i) as *const __m128i);
        let dp = dst.add(i) as *mut __m128i;
        let a = _mm_and_si128(s, alpha);
        let opaque = _mm_movemask_epi8(_mm_cmpeq_epi32(a, alpha));
        let clear = _mm_cmpeq_epi32(a, _mm_setzero_si128());
        if opaque == 0xFFFF {
            _mm_storeu_si128(dp, s);
        } else if _mm_movemask_epi8(clear) != 0xFFFF {
            let d = _mm_loadu_si128(dp);
            let lo = mix2(
                _mm_unpacklo_epi8(s, _mm_setzero_si128()),
                _mm_unpacklo_epi8(d, _mm_setzero_si128()),
            );
            let hi = mix2(
                _mm_unpackhi_epi8(s, _mm_setzero_si128()),
                _mm_unpackhi_epi8(d, _mm_setzero_si128()),
            );
            let mixed = _mm_or_si128(_mm_packus_epi16(lo, hi), alpha);
            // A clear pixel keeps what was there, alpha and all.
            _mm_storeu_si128(
                dp,
                _mm_or_si128(_mm_and_si128(clear, d), _mm_andnot_si128(clear, mixed)),
            );
        }
        i += 16;
    }
}

/// Two pixels as 16-bit channels: `(x + 1 + (x >> 8)) >> 8`, `x = s * a + d * (255 - a)`.
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn mix2(s: __m128i, d: __m128i) -> __m128i {
    let a = _mm_shufflehi_epi16::<0xFF>(_mm_shufflelo_epi16::<0xFF>(s));
    let ia = _mm_sub_epi16(_mm_set1_epi16(255), a);
    let x = _mm_add_epi16(_mm_mullo_epi16(s, a), _mm_mullo_epi16(d, ia));
    let x = _mm_add_epi16(_mm_add_epi16(x, _mm_srli_epi16::<8>(x)), _mm_set1_epi16(1));
    _mm_srli_epi16::<8>(x)
}
