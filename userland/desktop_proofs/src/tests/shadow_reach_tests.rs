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

//! The dock is drawn on the transparent chrome with the toolkit's own shadow
//! and panel, then hidden. Whatever pixel the drawing touched must lie in
//! the damage the shell commits, or the compositor never recomposes it and
//! the dock's outline stays on screen after the dock went.

use nonos_app_skeleton::PaintBuffer;

use crate::shadow_reach::with_shadow;

const W: u32 = 200;
const H: u32 = 120;
const DOCK: (u32, u32, u32, u32) = (40, 80, 120, 32);
const SPREADS: [u32; 4] = [3, 4, 5, 6];

/// The pixels a shown dock changes on the transparent chrome.
fn painted(spread: u32) -> Vec<(u32, u32)> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    let (x, y, w, h) = DOCK;
    fb.shadow_round(x, y, w, h, 16, spread, 0x5A00_0000);
    fb.panel(x, y, w, h, 16, 0xB810_1A22, 0x2922_C3F0);
    (0..W * H).filter(|&i| px[i as usize] != 0).map(|i| (i % W, i / W)).collect()
}

fn inside(p: (u32, u32), r: (u32, u32, u32, u32)) -> bool {
    p.0 >= r.0 && p.0 < r.0 + r.2 && p.1 >= r.1 && p.1 < r.1 + r.3
}

#[test]
fn the_dock_damage_covers_its_shadow() {
    for spread in SPREADS {
        let damage = with_shadow(DOCK, spread, W, H);
        for p in painted(spread) {
            assert!(
                inside(p, damage),
                "spread {spread}: ({}, {}) is drawn but not damaged",
                p.0,
                p.1
            );
        }
    }
}

#[test]
fn the_dock_rectangle_alone_left_the_shadow_behind() {
    let outside = painted(3).into_iter().filter(|&p| !inside(p, DOCK)).count();
    assert!(outside > 0, "the shadow reaches past the dock's rectangle");
}

#[test]
fn the_damage_stays_on_the_display() {
    assert_eq!(with_shadow((0, 100, 50, 20), 6, 50, 120), (0, 94, 50, 26));
    assert_eq!(with_shadow((10, 10, 0, 0), 2, 50, 50), (8, 8, 4, 4));
}
