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

//! What the compositor asks, as a driver reads it: FLUSH and CURSOR bodies
//! of every length and count, held to the test's own reading. The
//! compositor is input to a driver.

use super::body_lies::near;
use super::frames::{bytes, le16, le32};
use super::rng::Rng;
use crate::body::{cursor::Cursor, flush::Flush};

#[test]
fn flush_and_cursor_are_taken_exactly_as_the_test_reads_them() {
    let mut r = Rng::new(0xF1_0511);
    let (mut seen_f, mut seen_c) = ([0usize; 2], [0usize; 2]);
    for _ in 0..200_000 {
        let n = [r.below(16), r.below(40), r.word()][r.below(3) as usize];
        let mut b = r.word().to_le_bytes().to_vec();
        b.extend(n.to_le_bytes());
        let m = near(&mut r, 16 * (n as usize).min(20));
        b.extend(bytes(&mut r, m));
        let model = (1..=15).contains(&n) && b.len() >= 8 + 16 * n as usize;
        let got = Flush::decode(&b);
        assert_eq!(got.is_ok(), model, "flush {n} {}", b.len());
        if let Ok(f) = got {
            assert!(f
                .as_slice()
                .iter()
                .enumerate()
                .all(|(i, x)| Some(x.height) == le32(&b, 20 + 16 * i)));
        }
        seen_f[model as usize] += 1;
        let m = near(&mut r, 20);
        let c = bytes(&mut r, m);
        let h = |i: usize| le16(&c, 12 + 2 * i).unwrap_or(0);
        let model =
            c.len() >= 20 && h(2) <= 64 && h(3) <= 64 && h(0) < h(2).max(1) && h(1) < h(3).max(1);
        assert_eq!(Cursor::decode(&c).is_ok(), model, "cursor {c:02X?}");
        seen_c[model as usize] += 1;
    }
    assert!(seen_f.iter().chain(&seen_c).all(|&n| n >= 5_000), "{seen_f:?} {seen_c:?}");
}
