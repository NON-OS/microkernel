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
//! The band's four marks as 10 by 10 glyphs, a row a word with the left
//! pixel in bit 9: a camera, a microphone, a screen and a lock.

pub type Glyph = [u16; 10];
pub const SIDE: u32 = 10;

/// A video camera: the body, the lens to its right.
pub const CAMERA: Glyph = [
    0b0000000000,
    0b0000000000,
    0b1111110001,
    0b1111110011,
    0b1111110111,
    0b1111110111,
    0b1111110011,
    0b1111110001,
    0b0000000000,
    0b0000000000,
];
/// A microphone on its stand.
pub const MICROPHONE: Glyph = [
    0b0001111000,
    0b0001111000,
    0b0001111000,
    0b0001111000,
    0b0101111010,
    0b0101111010,
    0b0010000100,
    0b0001111000,
    0b0000110000,
    0b0001111000,
];
/// A screen on its foot.
pub const SCREEN: Glyph = [
    0b1111111111,
    0b1000000001,
    0b1000000001,
    0b1000000001,
    0b1000000001,
    0b1000000001,
    0b1111111111,
    0b0000110000,
    0b0001111000,
    0b0000000000,
];
/// A padlock, for the trusted band.
pub const LOCK: Glyph = [
    0b0001111000,
    0b0010000100,
    0b0010000100,
    0b0010000100,
    0b1111111111,
    0b1111111111,
    0b1111001111,
    0b1111001111,
    0b1111111111,
    0b1111111111,
];
