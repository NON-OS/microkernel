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

use super::fixture::{ivrs, read};
use super::ivhd_scope::MAX_SPANS;

#[test]
fn hostile_tables_never_read_out_of_bounds() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    for len in 0..400 {
        let mut t = ivrs(0x10, &[]);
        for _ in 0..len {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            t.push(seed as u8);
        }
        let n = (t.len() as u16).to_le_bytes();
        t[50..52].copy_from_slice(&n);
        assert!(read(&t).len() <= MAX_SPANS);
        assert!(read(&t[..t.len() / 2]).len() <= MAX_SPANS);
    }
}
