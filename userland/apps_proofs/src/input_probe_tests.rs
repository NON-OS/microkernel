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

//! The input probe draws only the theme's roles, in every scheme and in high
//! contrast, and the keys it echoes read on its ground.

use nonos_toolkit::theme::contrast::{at_least, BODY};
use nonos_toolkit::theme::{from_theme, theme_of, SCHEMES};

use crate::input_probe::render::draw::draw;
use crate::input_probe::state::Context;

const W: u32 = 400;
const H: u32 = 64;
const KEYS: &[u8] = b"AZ09 ?";

fn echoed(px: &mut [u32]) -> Context {
    let mut ctx = Context::new(px.as_mut_ptr() as u64, W, H, W * 4, 1, 2);
    ctx.buf[..KEYS.len()].copy_from_slice(KEYS);
    ctx.cursor = KEYS.len();
    ctx
}

#[test]
fn every_pixel_is_a_role_and_the_keys_read_on_the_ground() {
    let schemes = (0..SCHEMES.len() as u8).map(|i| (i, false)).chain([(0, true)]);
    for (i, hc) in schemes {
        let r = from_theme(&theme_of(i, hc));
        let (ground, ink) = (r.bg.argb(), r.text.argb());
        let mut px = vec![0u32; (W * H) as usize];
        draw(&echoed(&mut px), &r);
        for (n, &c) in px.iter().enumerate() {
            let at = (n as u32 % W, n as u32 / W);
            assert!(c == ground || c == ink, "scheme {i} hc {hc}: {c:#010x} at {at:?}");
        }
        assert_eq!(px[0], ground, "scheme {i} hc {hc}: the ground");
        assert!(px.contains(&ink), "scheme {i} hc {hc}: no key was drawn");
        assert!(at_least(ink, ground, BODY), "scheme {i} hc {hc}: the keys on the ground");
    }
}
