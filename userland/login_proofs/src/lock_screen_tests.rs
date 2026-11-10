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

//! The lock screen draws only the theme's roles, in every scheme and in high
//! contrast, and its bar stands out from the ground.

use nonos_toolkit::decorations::TRANSPARENT;
use nonos_toolkit::theme::contrast::{at_least, LARGE};
use nonos_toolkit::theme::{from_theme, theme_of, SCHEMES};

use crate::locked::{draw_clear, draw_locked};
use crate::state::Context;

const W: u32 = 320;
const H: u32 = 200;

fn screen(px: &mut [u32]) -> Context {
    Context::new(1, 2, 3, W, H, W * 4, px.as_mut_ptr() as u64)
}

fn themes() -> impl Iterator<Item = (u8, bool)> {
    (0..SCHEMES.len() as u8).map(|i| (i, false)).chain([(0, true)])
}

#[test]
fn every_pixel_of_the_lock_screen_is_a_role_and_the_bar_stands_out() {
    for (i, hc) in themes() {
        let r = from_theme(&theme_of(i, hc));
        let (ground, bar) = (r.bg.argb(), r.accent.argb());
        let mut px = vec![0u32; (W * H) as usize];
        draw_locked(&screen(&mut px), &r);
        for (n, &c) in px.iter().enumerate() {
            let at = (n as u32 % W, n as u32 / W);
            assert!(c == ground || c == bar, "scheme {i} hc {hc}: {c:#010x} at {at:?}");
        }
        assert_eq!(px[0], ground, "scheme {i} hc {hc}: the ground");
        assert_eq!(px[(0x20 * W + W / 2) as usize], bar, "scheme {i} hc {hc}: the bar");
        assert!(at_least(bar, ground, LARGE), "scheme {i} hc {hc}: the bar on the ground");
    }
}

#[test]
fn no_lock_leaves_the_wallpaper_showing() {
    let mut px = vec![0xFFu32; (W * H) as usize];
    draw_clear(&screen(&mut px));
    assert!(px.iter().all(|&c| c == TRANSPARENT));
}
