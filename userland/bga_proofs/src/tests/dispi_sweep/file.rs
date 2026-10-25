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

//! A DISPI register file as an adapter might leave it, and an adapter that
//! may answer a mode written with another.

use nonos_devmodel::FakeBar;

use crate::dispi::dispi_off;
use crate::tests::wire_sweep::rng::Rng;

/// The registers read at start-up, in DISPI index order from XRES (1) to
/// VIDEO_MEMORY_64K (0xA): xres, yres, bpp, enable, bank, virt_width, (7),
/// x_offset, y_offset, video memory in 64 KiB units.
pub type File = [u16; 10];

fn one(r: &mut Rng, edges: &[u16]) -> u16 {
    if r.below(5) == 0 {
        r.next() as u16
    } else {
        edges[r.below(edges.len() as u32) as usize]
    }
}

pub fn draw(r: &mut Rng) -> File {
    let sides =
        [0, 1, 16, 480, 600, 640, 720, 768, 800, 1024, 1080, 1280, 1920, 2560, 4096, 0xFFFF];
    let mut f = [0u16; 10];
    (f[0], f[1]) = (one(r, &sides), one(r, &sides));
    f[2] = one(r, &[0, 8, 15, 16, 24, 32, 32, 32]);
    f[3] = one(r, &[0, 0x01, 0x40, 0x41, 0xC1, 0x43, 0x41]);
    f[4] = one(r, &[0, 1]);
    f[5] = one(r, &[0, f[0], f[0].wrapping_add(8), 4096, 0xFFFF]);
    (f[7], f[8]) = (one(r, &[0, 0, 0, 1, 768]), one(r, &[0, 0, 0, 1, 600]));
    f[9] = one(r, &[0, 1, 16, 48, 128, 256, 0xFFFF]);
    if r.below(3) == 0 {
        // As firmware leaves it: shown linear at 32 bits, lines the width.
        (f[2], f[3], f[5], f[7], f[8], f[9]) = (32, 0x41, f[0], 0, 0, 256);
    }
    f
}

/// A register window holding `f`.
pub fn window(f: &File) -> FakeBar {
    let bar = FakeBar::new(0x600);
    for (i, v) in f.iter().enumerate() {
        bar.present16(dispi_off(i as u32 + 1) as usize, *v);
    }
    bar
}
