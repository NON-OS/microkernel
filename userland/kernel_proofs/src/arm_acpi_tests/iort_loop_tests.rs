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

//! IORT mappings that loop, and a range whose output would pass u32.

use super::iort_build::*;
use crate::arm_acpi::iort::{iort, translate};

#[test]
fn mappings_that_loop_end() {
    let a = node(
        1,
        2,
        0,
        &[vec![0; 13], b"A\0".to_vec()].concat(),
        &[mapping((0, 0xFFFF_FFFF, 0, 48 + 51, 0))],
    );
    let b = node(
        1,
        2,
        1,
        &[vec![0; 13], b"B\0".to_vec()].concat(),
        &[mapping((0, 0xFFFF_FFFF, 0, 48, 0))],
    );
    let n = iort(&iort_of(&[a, b])).unwrap();
    assert_eq!(translate(&n, 48, 3), None);
    let wide = node(
        1,
        2,
        0,
        &[vec![0; 13], b"C\0".to_vec()].concat(),
        &[mapping((10, u32::MAX, u32::MAX - 1, 48, 0))],
    );
    let n = iort(&iort_of(&[wide])).unwrap();
    assert_eq!(n[0].mappings[0].map(12), None, "an output past u32 is no output");
}
