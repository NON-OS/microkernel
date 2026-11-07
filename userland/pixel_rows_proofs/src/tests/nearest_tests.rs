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

use crate::nearest::{nearest_row, NearestCols};
use crate::reference::resample;
use crate::rng::Rng;

/// The old whole-frame resample, rebuilt from rows.
fn by_rows(src: &[u8], (sw, sh, stride): (u32, u32, u32), (dw, dh): (u32, u32)) -> Vec<u8> {
    let cols = NearestCols::new(sw, dw).expect("a table");
    let mut out = vec![0u8; (dw * dh * 4) as usize];
    for (y, row) in out.chunks_exact_mut(dw as usize * 4).enumerate() {
        let sy = ((2 * y + 1) * sh as usize / (2 * dh as usize)).min(sh as usize - 1);
        let from = sy * stride as usize;
        nearest_row(row, &src[from..from + sw as usize * 4], &cols, 0);
    }
    out
}

#[test]
fn rows_scale_as_the_old_resample_did() {
    let mut r = Rng::new(23);
    for (sw, sh, dw, dh) in [
        (4, 4, 2, 2),
        (4, 4, 8, 8),
        (7, 5, 13, 3),
        (100, 60, 150, 90),
        (150, 90, 100, 60),
        (1, 1, 9, 9),
    ] {
        let stride = sw * 4 + 12;
        let src = r.pixels((stride / 4 * sh) as usize);
        let mut want = vec![0u8; (dw * dh * 4) as usize];
        assert!(resample(&src, (sw, sh, stride), &mut want, (dw, dh)));
        assert_eq!(by_rows(&src, (sw, sh, stride), (dw, dh)), want, "{sw}x{sh} to {dw}x{dh}");
    }
}

#[test]
fn a_span_starts_at_its_column() {
    let src: Vec<u8> = (0..10u32).flat_map(|v| v.to_le_bytes()).collect();
    let cols = NearestCols::new(10, 20).expect("a table");
    let mut dst = vec![0u8; 4 * 4];
    nearest_row(&mut dst, &src, &cols, 6);
    let got: Vec<u32> =
        dst.chunks_exact(4).map(|p| u32::from_le_bytes([p[0], p[1], p[2], p[3]])).collect();
    assert_eq!(got, [3, 3, 4, 4]);
}

#[test]
fn an_empty_side_has_no_table_and_a_short_source_writes_nothing_past_it() {
    assert!(NearestCols::new(0, 4).is_none());
    assert!(NearestCols::new(4, 0).is_none());
    let cols = NearestCols::new(8, 8).expect("a table");
    let mut dst = vec![0xAAu8; 32];
    nearest_row(&mut dst, &[1, 2, 3, 4, 5, 6, 7, 8], &cols, 0);
    assert_eq!(&dst[..8], &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(dst[8..].iter().all(|&b| b == 0xAA));
}
