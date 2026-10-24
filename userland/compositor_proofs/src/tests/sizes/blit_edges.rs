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

//! The fitted box and its samples at every edge.

use super::edges::pairs;
use super::Pick;
use crate::state::damage::Rect;
use crate::sw_blitter::fit::{fit_box, fitted};
use crate::sw_blitter::resample::centre;

#[test]
fn the_fitted_box_agrees_with_u128_and_stays_on_the_output() {
    let mut p = Pick(0x7000_F17B);
    for ((sw, sh), _) in pairs().zip(0..) {
        for _ in 0..40 {
            let dst = (p.edge(), p.edge());
            let (s, d) = ((sw as u128, sh as u128), (dst.0 as u128, dst.1 as u128));
            let got = fit_box((sw, sh), dst);
            if s.0 > 0 && s.1 > 0 {
                let (bw, bh) = match d.0 * s.1 <= d.1 * s.0 {
                    true => (d.0, s.1 * d.0 / s.0),
                    false => (s.0 * d.1 / s.1, d.1),
                };
                let want = (((d.0 - bw) / 2) as u32, ((d.1 - bh) / 2) as u32, bw as u32, bh as u32);
                assert_eq!(got, want, "{sw}x{sh} on {dst:?}");
            } else {
                assert_eq!(got, (0, 0, 0, 0));
            }
            let r = Rect { x: p.edge(), y: p.edge(), width: p.edge(), height: p.edge() };
            if let Some(f) = fitted((sw, sh), dst, r) {
                assert!(
                    f.x as u64 + f.width as u64 <= dst.0 as u64,
                    "{:?} {:?}",
                    (r.x, r.y, r.width, r.height),
                    (f.x, f.y, f.width, f.height)
                );
                assert!(
                    f.y as u64 + f.height as u64 <= dst.1 as u64,
                    "{:?} {:?}",
                    (r.x, r.y, r.width, r.height),
                    (f.x, f.y, f.width, f.height)
                );
            }
        }
    }
}

#[test]
fn each_sample_lies_in_its_source_and_never_goes_back() {
    for (src, len) in pairs().filter(|&(s, l)| s > 0 && l > 0) {
        let (src, len) = (src as u64, len as u64);
        let mut last = 0;
        let mut ds = [0, 1, len / 2, len.saturating_sub(2), len - 1].map(|d| d.min(len - 1));
        ds.sort();
        for d in ds {
            let c = centre(d, src, len);
            assert!(c < src && c >= last, "{d} of {len} from {src}: {c}");
            last = c;
        }
    }
}
