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

//! Q11 load averages and the load row.

use crate::paint::rail_fmt::{load3_into, load_q11_into};

fn q11(v: u64) -> String {
    let mut b = [0u8; 48];
    let n = load_q11_into(&mut b, v);
    core::str::from_utf8(&b[..n]).unwrap().to_string()
}

#[test]
fn a_q11_load_reads_as_a_two_place_decimal() {
    assert_eq!(q11(0), "0.00", "an idle scheduler is a measured zero");
    assert_eq!(q11(2048), "1.00", "2048 is the fixed-point unit");
    assert_eq!(q11(860), "0.42");
    assert_eq!(q11(1024), "0.50");
    assert_eq!(q11(20 * 2048 + 512), "20.25");
}

#[test]
fn a_q11_load_rounds_to_the_nearest_hundredth_rather_than_truncating() {
    assert_eq!(q11(10), "0.00", "just under half a hundredth rounds down");
    assert_eq!(q11(11), "0.01", "just over it rounds up");
    assert_eq!(q11(2047), "1.00", "one unit short of 1.00 still reads as 1.00");
}

#[test]
fn a_large_q11_load_still_writes_its_whole_integer_part() {
    assert_eq!(q11(1024 * 2048), "1024.00");
    assert_eq!(q11(u64::MAX), "90071992547409.91", "saturation, never a wrap");
}

#[test]
fn the_load_row_is_the_three_averages_in_order() {
    let mut b = [0u8; 48];
    let n = load3_into(&mut b, [860, 635, 573]);
    assert_eq!(core::str::from_utf8(&b[..n]).unwrap(), "0.42 0.31 0.28");
    let n = load3_into(&mut b, [0, 2048, 4096]);
    assert_eq!(core::str::from_utf8(&b[..n]).unwrap(), "0.00 1.00 2.00");
}
