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

//! The pixel bytes a frame moves inside the bridge, apart from those read
//! out of the guest: laid into the kept client pixels, into the window, or
//! brought up from the buffer shown. Pure, so the host proofs hold it.

/// The bytes of a `w` by `h` box of four-byte pixels, saturating.
pub fn box_bytes(w: u32, h: u32) -> u64 {
    u64::from(w).saturating_mul(u64::from(h)).saturating_mul(4)
}

/// The bytes `lay` (damage_lay.rs) moves for the box with corners
/// (x0, y0, x1, y1): none for one empty or turned inside out.
pub fn corner_bytes((x0, y0, x1, y1): (u32, u32, u32, u32)) -> u64 {
    box_bytes(x1.saturating_sub(x0), y1.saturating_sub(y0))
}
