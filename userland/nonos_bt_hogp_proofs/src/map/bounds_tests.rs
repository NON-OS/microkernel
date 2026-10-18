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

//! Sizes, IDs, report lengths and counts a peripheral could push past
//! the host's bounds, or past what the arithmetic holds.

use super::maps::{bits_field, global, item, local, main};
use nonos_bt_hogp::report::map::{parse, MapError::*};

#[test]
fn sizes_ids_and_reports_out_of_bounds() {
    assert_eq!(parse(&global(7, 33)), Err(Value), "Report Size past 32");
    assert_eq!(parse(&global(8, 0)), Err(Value), "Report ID 0");
    assert_eq!(parse(&item(1, 0, &[0, 0, 1, 0])), Err(Value), "a page past 16 bits");
    let wide = |count: &[u8]| [global(7, 32), item(1, 9, count), main(8, 1)].concat();
    assert!(parse(&wide(&[16])).is_ok(), "512 bits");
    assert_eq!(parse(&wide(&[17])), Err(TooMany));
    assert_eq!(parse(&wide(&[0xff, 0xff, 0xff, 0xff])), Err(TooMany), "no overflow");
    // 32 times this count is 32 more than 2^32: a 32-bit product wraps.
    assert_eq!(parse(&wide(&[0x01, 0x00, 0x00, 0x08])), Err(TooMany), "no wrap");
    let mixed = [bits_field(0, 0xe0, 8), bits_field(1, 0xe0, 8)].concat();
    assert_eq!(parse(&mixed), Err(Shape), "numbered and unnumbered reports");
    let max_under = [global(1, 5), global(2, 4), local(0, 4), global(7, 8), global(9, 1)];
    assert_eq!(parse(&[max_under.concat(), main(8, 0)].concat()), Err(Value));
}

#[test]
fn fields_and_reports_past_their_bounds() {
    let one = [local(0, 4), main(8, 2)].concat();
    let many = |n: usize| [bits_field(1, 0xe0, 1), one.repeat(n - 1)].concat();
    assert!(parse(&many(64)).is_ok());
    assert_eq!(parse(&many(65)), Err(TooMany));
    let ids = |n: u8| (1..=n).map(|id| bits_field(id, 0xe0, 1)).collect::<Vec<_>>().concat();
    assert!(parse(&ids(16)).is_ok());
    assert_eq!(parse(&ids(17)), Err(TooMany));
}
