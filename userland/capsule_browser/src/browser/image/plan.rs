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

/* The most pixels one stored raster may hold (16 MiB of ARGB). */
pub(super) const MAX_RASTER_PX: u64 = 4_194_304;

/// The size to keep an image of natural size `nat` at: just large enough
/// to cover the largest box it is drawn into (`hint`, a zero side meaning
/// unknown), never larger than natural, and at most `max_px` pixels, the
/// aspect ratio kept throughout.
pub(super) fn target(nat: (u32, u32), hint: (u32, u32), max_px: u64) -> (u32, u32) {
    let (w, h) = (nat.0.max(1) as f64, nat.1.max(1) as f64);
    let fit = |want: u32, have: f64| if want == 0 { 1.0 } else { want as f64 / have };
    let mut s = match hint {
        (0, 0) => 1.0,
        (hw, 0) => fit(hw, w),
        (0, hh) => fit(hh, h),
        (hw, hh) => fit(hw, w).max(fit(hh, h)),
    }
    .min(1.0);
    let px = w * h * s * s;
    if px > max_px as f64 {
        s *= sqrt(max_px as f64 / px);
    }
    let side = |v: f64| ceil(v * s).max(1);
    let (tw, th) = (side(w), side(h));
    /* Rounding up must not break the pixel cap. */
    if tw as u64 * th as u64 > max_px {
        return ((tw - 1).max(1), (th - 1).max(1));
    }
    (tw, th)
}

pub(super) fn ceil(v: f64) -> u32 {
    let t = v as u32;
    t + u32::from((t as f64) < v)
}

/* Newton square root for a positive ratio; core has no sqrt. */
fn sqrt(x: f64) -> f64 {
    let mut r = if x > 1.0 { x } else { 1.0 };
    for _ in 0..40 {
        r = 0.5 * (r + x / r);
    }
    r
}
