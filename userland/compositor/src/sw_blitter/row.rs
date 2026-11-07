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

//! Compositing one row of a layer onto the frame. Holds no surface, so
//! `mechanism_proofs` includes it and checks it pixel for pixel.

/// Source-over one row of ARGB pixels onto `dst`, by the layer rule
/// (`nonos_pixel_rows::composite`): opaque runs copied whole, clear pixels
/// skipped, the rest mixed; AVX2 or SSE2 where the processor has them. A
/// layer is opaque across almost its whole width, and this is every window
/// of the desktop on the software path.
pub fn composite_row(src: &[u32], dst: &mut [u32]) {
    let src = nonos_pixel_rows::words::bytes(src);
    nonos_pixel_rows::composite_row(nonos_pixel_rows::words::bytes_mut(dst), src);
}

/// An XRGB row put down whole, every pixel opaque whatever its fourth byte.
pub fn opaque_row(src: &[u32], dst: &mut [u32]) {
    for (d, s) in dst.iter_mut().zip(src) {
        *d = *s | 0xFF00_0000;
    }
}
