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

/* One box-shadow layer: offsets, blur radius and spread in px, the ARGB
 * colour, and whether it is cast inside the padding box. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ShadowLayer {
    pub dx: i16,
    pub dy: i16,
    pub blur: u16,
    pub spread: i16,
    pub color: u32,
    pub inset: bool,
}

/* box-shadow keeps this many visible layers, the first painted on top. */
pub const MAX_SHADOWS: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Shadow {
    pub layers: [ShadowLayer; MAX_SHADOWS],
    pub n: u8,
}
