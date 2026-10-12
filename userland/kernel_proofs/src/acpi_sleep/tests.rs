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

use super::sleep_request::{compare, valid, Before, SOFT_OFF};

#[test]
fn soft_off_with_three_bit_values_only() {
    assert!(valid(SOFT_OFF, 5, 5));
    assert!(valid(SOFT_OFF, 0, 7));
    assert!(!valid(SOFT_OFF, 8, 0), "four bits");
    assert!(!valid(SOFT_OFF, 0, 8));
    assert!(!valid(3, 5, 5), "S3 waits on a resume path");
    assert!(!valid(4, 5, 5));
}

#[test]
fn what_the_constant_reader_had() {
    assert_eq!(compare(None, (5, 5)), Before::None);
    assert_eq!(compare(Some((5, 5)), (5, 5)), Before::Same);
    assert_eq!(compare(Some((7, 0)), (5, 5)), Before::Differed(7, 0));
}
