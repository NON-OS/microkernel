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

//! The installer's proof rows and key/value lines cut by the toolkit's
//! `clip_by` over the brand's body measure, where its private `fit` cut them.

use nonos_toolkit::widgets::text::clip_by;

use crate::installer::text::body_w;
use crate::installer_layout_tests::bodies;
use crate::{measure, Face};

const VALUES: [&str; 10] = [
    "kernel signature",
    "signed by the keys this development loader holds",
    "the boot records disagree on what was proven",
    "root 3f9a0c1e  kernel 77b2d4e09a1c",
    "WDC WDS100T2B0C-00PXH0 NVMe 1TB SSD 0123",
    "1000.2 GB",
    "INSTALL N\u{d8}NOS \u{2713} caf\u{e9} \u{fc}n\u{ef}code",
    "a  double  spaced   line ",
    "W",
    "",
];

fn old_fit(s: &str, px: f32, max_w: u32) -> &str {
    let mut end = s.len();
    while end > 0 {
        if s.is_char_boundary(end) && measure(&s[..end], Face::Body, px, 0.0) <= max_w {
            return &s[..end];
        }
        end -= 1;
    }
    ""
}

#[test]
fn every_value_cuts_where_the_old_fit_cut_it() {
    let mut sizes: Vec<f32> = Vec::new();
    for b in bodies() {
        let m = b.m;
        sizes.extend([m.body_px, m.small_px, m.mono_px, m.mono_small_px]);
    }
    sizes.sort_by(f32::total_cmp);
    sizes.dedup();
    assert!(sizes.len() >= 2, "{sizes:?}");
    let mut checked = 0u32;
    for s in VALUES {
        for &px in &sizes {
            let full = measure(s, Face::Body, px, 0.0);
            for max_w in 0..=full + 4 {
                let cut = clip_by(s, max_w as i32, body_w(px));
                assert_eq!(cut, old_fit(s, px, max_w), "{s:?} at {px} in {max_w}");
                checked += 1;
            }
        }
    }
    assert!(checked > 1000, "{checked}");
}
