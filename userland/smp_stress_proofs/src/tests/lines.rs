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

//! The run length read from the command line, and the lines printed.

use crate::line::{Line, CAP};
use crate::number::first_number;

#[test]
fn the_run_length_is_the_first_number_on_the_command_line() {
    assert_eq!(first_number(b"smp_stress\x00600"), Some(600));
    assert_eq!(first_number(b"smp_stress 3600 extra 9"), Some(3600));
    assert_eq!(first_number(b"smp_stress"), None);
    assert_eq!(first_number(b"0"), None);
    assert_eq!(first_number(b"99999999999999999999999"), Some(u64::MAX));
}

#[test]
fn an_hour_at_a_million_rounds_a_second_still_fits_the_final_line() {
    let big = 3_600_000_000;
    let mut line = Line::tagged();
    line.str(b" FAIL").field(b"ran_ms", 3_600_000).field(b"left", 16);
    for key in [&b"futex"[..], b"ipc", b"sleeps", b"stalls", b"late", b"errors"] {
        line.field(key, big);
    }
    line.field(b"max_futex_ms", 2000).field(b"max_ipc_ms", 2000);
    line.field(b"max_sleep_over_ms", 2000);
    assert!(line.bytes().len() < CAP);
    assert!(line.bytes().ends_with(b"max_sleep_over_ms=2000"));
}

#[test]
fn numbers_print_in_decimal_and_a_line_never_passes_its_cap() {
    let mut line = Line::tagged();
    line.field(b"a", 0).field(b"b", u64::MAX);
    assert_eq!(line.bytes(), b"[SMP-STRESS] a=0 b=18446744073709551615");
    for _ in 0..100 {
        line.field(b"key", u64::MAX);
    }
    assert_eq!(line.bytes().len(), CAP);
}
