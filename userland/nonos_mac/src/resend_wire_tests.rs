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

//! The request's number, and a driver's error, through the resend rule.

use super::resend::{asked, serve, Kept};
use super::resend_cell::KeptCell;

#[test]
fn the_number_is_the_four_byte_body() {
    assert_eq!(asked(&7u32.to_le_bytes()), Some(7));
    assert_eq!(asked(&[]), None);
    assert_eq!(asked(&[1, 2]), None);
}

#[test]
fn a_driver_error_passes_through_and_keeps_nothing() {
    let mut kept = Kept::<4>::new();
    let r: Result<_, i32> = serve(&mut kept, Some(1), |_| Err(-5));
    assert_eq!(r.err(), Some(-5));
}

#[test]
fn the_cell_keeps_across_calls_and_a_busy_cell_keeps_nothing() {
    static CELL: KeptCell<4> = KeptCell::new();
    let fresh = |b: &mut [u8]| -> Result<Option<usize>, ()> { (b[0] = 9, Ok(Some(1))).1 };
    assert_eq!(CELL.with(|k| serve(k, Some(3), fresh).unwrap().map(|f| f[0])), Some(Some(9)));
    let again = CELL.with(|k| serve(k, Some(3), |_| Ok::<_, ()>(None)).unwrap().map(|f| f[0]));
    assert_eq!(again, Some(Some(9)), "kept between calls");
    let inner = CELL.with(|_| CELL.with(|_| ()));
    assert_eq!(inner, Some(None), "a cell in use refuses a second borrow");
}
