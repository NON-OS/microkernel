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

//! Arbitrary RX transfers up to BULK_MAX: the walk never reads outside
//! the bytes that came.

use crate::r8153::rx::{frames, RX_DESC};

#[test]
fn no_transfer_makes_the_walk_read_outside_it() {
    let mut x = 0x2545_f491_4f6c_dd1du64;
    for _ in 0..20_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let len = (x % 4097) as usize;
        let t: Vec<u8> = (0..len).map(|i| (x >> (i % 56)) as u8).collect();
        let mut seen = 0;
        let n = frames(&t, |f| seen += f.len());
        assert!(seen <= t.len() && n <= t.len() / (RX_DESC + 60) + 1);
    }
}
