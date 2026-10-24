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

//! A primary's length at u32's edges, against the test's u128
//! reading (the rows a copy writes: `copy_size_tests.rs`).

use crate::primary::{Primary, CURSOR_SLOT};
use crate::sweep::edges::pairs;

#[test]
fn a_primary_s_length_is_exact_or_refused_at_every_edge() {
    let mut seen = [0; 2];
    for ((w, h), cursor) in pairs().flat_map(|p| [(p, false), (p, true)]) {
        let frame = w as u128 * 4 * h as u128;
        let fits = w as u128 * 4 <= u32::MAX as u128 && (!cursor || frame <= u32::MAX as u128);
        let slot = if cursor { CURSOR_SLOT as u128 } else { 0 };
        let offset = if cursor { frame as u32 } else { 0 };
        let want = fits.then(|| ((frame + slot) as usize, w.wrapping_mul(4), offset));
        assert_eq!(Primary::len(w, h, cursor), want, "{w}x{h} cursor {cursor}");
        seen[want.is_some() as usize] += 1;
    }
    assert!(seen.iter().all(|&c| c > 100), "{seen:?}");
}
