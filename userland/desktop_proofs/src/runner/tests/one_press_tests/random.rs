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

//! Random presses and drags keep what is shown and what is hit the same.

use super::super::super::desk::Desk;

/// Three windows and every press anywhere, then a drag of whatever was hit:
/// over long random sessions the screen never shows a stale pixel, and never
/// shows one window where a press would land on another.
#[test]
fn random_presses_and_drags_keep_what_is_shown_and_what_is_hit_the_same() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    let mut below = |n: u32| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % n as u64) as u32
    };
    for session in 0..12 {
        let mut d = Desk::new(640, 400);
        for pid in 1..=4u32 {
            let (w, h) = (120 + below(260), 90 + below(200));
            let acc = if below(2) == 0 { 0 } else { 60 + below(80) };
            d.open(pid, (below(640 - w), 46 + below(354 - h), w, h), acc);
        }
        for step in 0..40 {
            let (x, y) = (below(640), below(400));
            d.press(x, y);
            let (mut px, mut py) = (x as i32, y as i32);
            for _ in 0..below(6) {
                px = (px + below(81) as i32 - 40).clamp(0, 639);
                py = (py + below(81) as i32 - 40).clamp(0, 399);
                d.motion(px as u32, py as u32);
            }
            d.release(px as u32, py as u32);
            d.assert_consistent(&format!("session {session}, step {step}"));
        }
    }
}
