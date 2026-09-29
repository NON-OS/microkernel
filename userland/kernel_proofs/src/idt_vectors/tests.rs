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

use super::vectors::{exception_has_error_code, exception_name};

/* Intel SDM volume 3, table 6-1, and AMD APM volume 2, table 8-1. */
const PUSHES_ERROR_CODE: [u8; 10] = [8, 10, 11, 12, 13, 14, 17, 21, 29, 30];

#[test]
fn error_code_table_is_the_architectural_one() {
    for v in 0..=u8::MAX {
        assert_eq!(
            exception_has_error_code(v),
            PUSHES_ERROR_CODE.contains(&v),
            "vector {v}"
        );
    }
}

#[test]
fn vc_and_sx_are_named() {
    assert_eq!(exception_name(29), "VMM Communication Exception");
    assert_eq!(exception_name(30), "Security Exception");
}
