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

//! MODES as the compositor reads it: a count past twenty, or past the
//! bytes that follow it, is refused; every other is read mode for mode.

use super::body_lies::near;
use super::frames::{bytes, le32};
use super::rng::Rng;
use crate::body::modes::Modes;

#[test]
fn a_mode_list_is_taken_exactly_as_the_test_reads_it() {
    let mut r = Rng::new(4);
    let (mut t, mut f) = (0, 0);
    for _ in 0..100_000 {
        let k = [r.below(21), r.below(30), r.word()][r.below(3) as usize];
        let mut b = k.to_le_bytes().to_vec();
        let m = near(&mut r, 12 * (k as usize).min(30));
        b.extend(bytes(&mut r, m));
        let model = k <= 20 && b.len() >= 4 + 12 * k as usize;
        let got = Modes::decode(&b);
        assert_eq!(got.is_ok(), model, "modes {k} {}", b.len());
        if let Ok(m) = got {
            let at = |i: usize| le32(&b, 4 + 12 * i);
            assert!(m.as_slice().iter().enumerate().all(|(i, x)| Some(x.width) == at(i)));
        }
        (t, f) = (t + model as usize, f + !model as usize);
    }
    assert!(t >= 10_000 && f >= 10_000, "modes {t} {f}");
}
