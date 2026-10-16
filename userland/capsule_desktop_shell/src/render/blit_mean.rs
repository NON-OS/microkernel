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

//! One destination pixel of a scaled blit: the alpha-weighted mean of the
//! source pixels its span covers, in 16.16 fixed point.

/// The mean colour and alpha of `rgba` (`sw` x `sh`) over the source spans
/// `xs` and `ys`; None when nothing under them is opaque at all.
pub(super) fn area_mean(
    rgba: &[u8],
    sw: u32,
    sh: u32,
    (x0f, x1f): (u64, u64),
    (y0f, y1f): (u64, u64),
) -> Option<(u32, u32, u32, u32)> {
    let sy_last = core::cmp::min((y1f - 1) >> 16, sh as u64 - 1);
    let sx_last = core::cmp::min((x1f - 1) >> 16, sw as u64 - 1);
    let (mut sr, mut sg, mut sb, mut sa, mut wsum) = (0u64, 0u64, 0u64, 0u64, 0u64);
    for syy in (y0f >> 16)..=sy_last {
        let wy = core::cmp::min(y1f, (syy + 1) << 16) - core::cmp::max(y0f, syy << 16);
        let bse = (syy as usize * sw as usize) * 4;
        for sxx in (x0f >> 16)..=sx_last {
            let wx = core::cmp::min(x1f, (sxx + 1) << 16) - core::cmp::max(x0f, sxx << 16);
            let w = wx * wy;
            let i = bse + sxx as usize * 4;
            let a = rgba[i + 3] as u64;
            sr += rgba[i] as u64 * a * w;
            sg += rgba[i + 1] as u64 * a * w;
            sb += rgba[i + 2] as u64 * a * w;
            sa += a * w;
            wsum += w;
        }
    }
    if wsum == 0 || sa == 0 {
        return None;
    }
    let a = (sa / wsum) as u32;
    let r = ((sr / sa) as u32).min(255);
    let g = ((sg / sa) as u32).min(255);
    let b = ((sb / sa) as u32).min(255);
    Some((r, g, b, a))
}
