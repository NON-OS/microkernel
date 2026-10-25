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

//! The boot framebuffer's sizes at u32's edges: taken exactly when
//! the test's u128 reading says, and its surface budget then exact.

#[path = "../../../gfx_serve_proofs/src/sweep/edges.rs"]
mod edges;

use crate::describe::caps;
use crate::fb::{Fb, Layout};
use edges::{pairs, EDGES};

const LENS: [usize; 5] = [0, 4096, 1 << 32, 1 << 40, usize::MAX];

#[test]
fn a_framebuffer_is_taken_exactly_when_it_fits_and_its_budget_is_exact() {
    let mut mem = vec![0u32; 16];
    let p = mem.as_mut_ptr() as *mut u8;
    let mut taken = 0;
    for ((w, h), (&stride, &len)) in
        pairs().flat_map(|p| EDGES.iter().flat_map(move |s| LENS.iter().map(move |l| (p, (s, l)))))
    {
        let line = w as u128 * 4;
        let need = stride as u128 * h.saturating_sub(1) as u128 + line;
        let ok = w > 0 && h > 0 && stride as u128 >= line && need <= len as u128;
        let fb = Fb::new(p, len, (w, h), stride, Layout::BlueFirst);
        assert_eq!(fb.is_some(), ok, "{w}x{h} stride {stride} len {len}");
        if let Some(f) = fb {
            assert_eq!(caps(&f).surface_budget as u128, w as u128 * h as u128 * 4);
            taken += 1;
        }
    }
    assert!(taken > 1_000, "{taken}");
}
