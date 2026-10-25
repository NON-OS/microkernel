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

//! The owner's measuring line: one bit for each bucket a call succeeded in.

use crate::fixture::hist;
use crate::market::buckets::{line, PREFIX};

#[test]
fn the_buckets_line_sets_a_bit_for_each_bucket_that_succeeded() {
    let l = line(&hist());
    assert_eq!(&l[..PREFIX.len()], PREFIX);
    let hex = core::str::from_utf8(&l[PREFIX.len()..]).expect("hex");
    let mut want = String::from("01");
    want += &"0".repeat(2 * 27);
    want += "80";
    want += &"0".repeat(130 - want.len());
    assert_eq!(hex, want, "buckets 0 and 231; refused writes and ptrace unset");
}
