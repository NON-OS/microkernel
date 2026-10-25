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

//! BGRT and the BMP it points at: sizes and offsets that lie, files cut
//! short, and headers whose rows would overflow the sum.

use super::gen::{damage, sdt, Rng};
use super::sweep::sweep;
use crate::bgrt::{bgrt_entry::bgrt, bmp_check::bmp, bmp_check::MAX_SIDE};

#[test]
fn bgrt_loads_only_a_whole_version_one_table() {
    let make = |r: &mut Rng| {
        let mut b = r.bytes(20);
        b[..2].copy_from_slice(&[r.below(3) as u8 & 1, 0]);
        damage(sdt(b"BGRT", 1, &b), r)
    };
    sweep(make, bgrt, |g, t| t.len() >= 56 && t[36] == 1 && g.address != 0 && g.orientation < 4);
}

fn file(r: &mut Rng) -> Vec<u8> {
    let side = |r: &mut Rng| {
        let any = r.next() as u32;
        r.pick(&[1, 2, 3, 16, MAX_SIDE, MAX_SIDE + 1, any])
    };
    let (w, h, bpp) = (side(r), side(r), r.pick(&[24u16, 32, 8, 1, 16]));
    let h = if r.one_in(2) { h.wrapping_neg() } else { h };
    let row = (u64::from(w) * u64::from(bpp) / 8).div_ceil(4) * 4;
    let rows = row.saturating_mul(u64::from((h as i32).unsigned_abs()));
    let size = rows.saturating_add(54).min(u64::from(u32::MAX)) as u32;
    let size = if r.one_in(4) { r.next() as u32 } else { size };
    let mut b = [&b"BM"[..], &size.to_le_bytes(), &[0; 4], &54u32.to_le_bytes()].concat();
    b.extend([40, w, h].map(u32::to_le_bytes).concat());
    b.extend([&1u16.to_le_bytes()[..], &bpp.to_le_bytes(), &[0; 24]].concat());
    b.extend(r.some_bytes(4096));
    b
}

#[test]
fn bmp_headers_hold_their_rows() {
    // An even length is handed over as the whole file.
    let whole = |b: &[u8]| b.len().is_multiple_of(2);
    sweep(
        file,
        |b| bmp(b, whole(b)),
        |m, b| {
            let row = (u64::from(m.width) * u64::from(m.bpp) / 8).div_ceil(4) * 4;
            m.width <= MAX_SIDE
                && (1..=MAX_SIDE).contains(&m.height)
                && u64::from(m.pixels_at) + row * u64::from(m.height) <= u64::from(m.file_size)
                && (!whole(b) || b.len() as u64 >= u64::from(m.file_size))
        },
    );
}
