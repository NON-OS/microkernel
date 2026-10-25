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
//! The panel with no record claims nothing, and no line of it is the same
//! in every state the boot can record.

use super::attest_lines_tests::{line, said};
use crate::attest_lines::lines;

const DIM: u32 = 0xFF5B_6B78;

#[test]
fn no_record_claims_nothing() {
    let s = lines(None).map(|(t, c)| (String::from_utf8(t.to_vec()).unwrap(), c));
    assert_eq!(s[0], line("[?] bootloader  not reported", DIM));
    assert_eq!(s[1], line("[?] kernel      not reported", DIM));
    assert_eq!(s[2], line("[?] capsules    not reported", DIM));
    assert_eq!(s[3], line("NO RECORD", DIM));
}

#[test]
fn every_line_follows_the_record() {
    for i in 0..4 {
        let all: Vec<_> = (0..27).map(|n| said(n / 9 % 3, n / 3 % 3, n % 3)[i].clone()).collect();
        assert!(all.iter().any(|l| *l != all[0]), "line {i} is the same in every state");
    }
}
