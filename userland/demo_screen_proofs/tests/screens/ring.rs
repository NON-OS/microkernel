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

//! About's Display tile draws a full ring in the accent colour with the
//! mode inside it. The value must sit in the ring's hole: one written
//! across the stroke breaks the ring, which this reads straight off the
//! frame.

/// About's accent (capsule_about's theme.rs).
const ACCENT: u32 = 0xFF35_C4E2;

/// Of the pixels well inside the stroke of the accent ring in the right
/// third of a frame `w` wide, the share that are still the accent, in
/// thousandths. None when no ring is found.
pub fn whole_per_mille(pixels: &[u32], w: u32) -> Option<u32> {
    let (x0, mut lo, mut hi) = (w * 2 / 3, (u32::MAX, u32::MAX), (0u32, 0u32));
    let at = |i: usize| ((i as u32) % w, (i as u32) / w);
    for (i, _) in pixels.iter().enumerate().filter(|(_, &p)| p == ACCENT) {
        let (x, y) = at(i);
        if x >= x0 {
            (lo, hi) = ((lo.0.min(x), lo.1.min(y)), (hi.0.max(x), hi.1.max(y)));
        }
    }
    if hi.0 <= lo.0 || hi.1 <= lo.1 {
        return None;
    }
    let (cx, cy) = ((lo.0 + hi.0) as f32 / 2.0, (lo.1 + hi.1) as f32 / 2.0);
    let outer = (hi.0 - lo.0) as f32 / 2.0;
    // The stroke is 7 px (TILE_RING_T); its middle 3 px have no edge blend.
    let (inner, outer) = (outer - 5.0, outer - 2.0);
    let (mut core, mut kept) = (0u32, 0u32);
    for y in lo.1..=hi.1 {
        for x in lo.0..=hi.0 {
            let d = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
            if d >= inner && d <= outer {
                core += 1;
                kept += (pixels[(y * w + x) as usize] == ACCENT) as u32;
            }
        }
    }
    (core > 0).then(|| kept * 1000 / core)
}
