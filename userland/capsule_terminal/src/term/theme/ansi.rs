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

/*
 * The sixteen colours programs name, chosen for the ground they are drawn
 * on. xterm's own set puts blue at #0000EE, which on a near-black window is
 * text nobody can read; a tool that prints its table headers in blue should
 * still be legible. The light set keeps each hue dark enough for a pale
 * ground. Indices 16 and up stay xterm's, since programs that use them pick
 * exact colours on purpose.
 */

const DARK: [u32; 16] = [
    0x2A2F37, 0xE8646E, 0x6FD48A, 0xE6C170, 0x5FA8F5, 0xC77DE0, 0x4FD2DC, 0xD4D8DF, 0x6B7380,
    0xFF7C86, 0x8FEBA6, 0xF3D68C, 0x86C1FF, 0xDB9CF0, 0x7AE6EE, 0xFFFFFF,
];

const LIGHT: [u32; 16] = [
    0x1F2328, 0xB3261E, 0x1A7F37, 0x8A6100, 0x0B5CD5, 0x8E3FB0, 0x0A7A83, 0x5A626C,
    0x57606A, 0xD1242F, 0x227E3C, 0x8E6800, 0x2C68DC, 0x974EB8, 0x157980, 0x24292F,
];

/// The set for a window whose ground is `bg` (0xAARRGGBB or 0xRRGGBB).
pub fn base16_for(bg: u32) -> &'static [u32; 16] {
    let (r, g, b) = ((bg >> 16) & 0xFF, (bg >> 8) & 0xFF, bg & 0xFF);
    // Rec. 601 luma, in 0..=255 scaled by 1000.
    let luma = 299 * r + 587 * g + 114 * b;
    if luma > 128_000 {
        &LIGHT
    } else {
        &DARK
    }
}
