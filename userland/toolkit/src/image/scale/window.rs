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

// The part of the source that fills the destination at its own aspect,
// centred, in 16.16 source pixels: (x0, y0, width, height).
pub fn cover_window(sw: u32, sh: u32, dw: u32, dh: u32) -> (u64, u64, u64, u64) {
    let (sw, sh, dw, dh) = (sw as u64, sh as u64, dw as u64, dh as u64);
    let full_w = sw << 16;
    let full_h = sh << 16;
    if sw * dh > sh * dw {
        // Source is wider: keep its height, crop the sides.
        let cw = (full_h * dw) / dh;
        ((full_w - cw) / 2, 0, cw, full_h)
    } else {
        let ch = (full_w * dh) / dw;
        (0, (full_h - ch) / 2, full_w, ch)
    }
}
