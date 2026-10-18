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

//! The modes BGA sets at u32's edges (G-70): taken exactly when the test's
//! u128 reading says, whatever the video memory's size.

#[path = "../../../gfx_serve_proofs/src/sweep/edges.rs"]
mod edges;

use crate::display::fits;
use edges::pairs;

#[test]
fn a_mode_fits_exactly_when_u128_says_at_every_edge() {
    let vram = [0usize, 16 << 20, 64 << 20, 1 << 32, usize::MAX];
    let mut taken = 0;
    for ((w, h), &len) in pairs().flat_map(|p| vram.iter().map(move |l| (p, l))) {
        let sides = (640..=4096).contains(&w) && (480..=4096).contains(&h);
        let ok = sides && w % 8 == 0 && w as u128 * h as u128 * 4 <= len as u128;
        assert_eq!(fits(len, w, h), ok, "{w}x{h} in {len}");
        taken += ok as u32;
    }
    assert!(taken > 10, "{taken}");
}
