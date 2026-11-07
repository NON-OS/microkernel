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

use crate::opaque::{opaque_row, scalar};
use crate::reference::make_opaque;
use crate::rng::Rng;

#[test]
fn every_path_sets_alpha_as_the_old_loop_did() {
    let mut r = Rng::new(17);
    for len in 0..70 {
        for off in 0..4 {
            let row = r.pixels(len + 1);
            let mut want = row[off..off + len * 4].to_vec();
            make_opaque(&mut want, len as u32, 1, (len * 4).max(1) as u32);
            let mut plain = row[off..off + len * 4].to_vec();
            scalar(&mut plain);
            assert_eq!(plain, want, "scalar");
            let mut picked = row[off..off + len * 4].to_vec();
            opaque_row(&mut picked);
            assert_eq!(picked, want, "the host's path");
            let mut v = row[off..off + len * 4].to_vec();
            // SAFETY: SSE2 is in every x86-64.
            let done = unsafe { crate::sse2::opaque(&mut v) };
            scalar(&mut v[done..]);
            assert_eq!(v, want, "sse2");
            if std::is_x86_feature_detected!("avx2") {
                let mut v = row[off..off + len * 4].to_vec();
                // SAFETY: checked just now.
                let done = unsafe { crate::avx2::opaque(&mut v) };
                scalar(&mut v[done..]);
                assert_eq!(v, want, "avx2");
            }
        }
    }
}

#[test]
fn a_ragged_row_keeps_its_last_bytes() {
    let mut row = vec![0u8; 4 * 9 + 3];
    opaque_row(&mut row);
    assert!(row[..36].chunks_exact(4).all(|p| p == [0, 0, 0, 0xFF]));
    assert_eq!(&row[36..], &[0, 0, 0], "not a whole pixel: untouched");
}
