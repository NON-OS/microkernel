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

//! The computed S-box against the standard table, for every input.

use super::nonos_sbox::sub_byte::sub_byte;
use super::sbox_table::REFERENCE_SBOX;

#[test]
fn nonos_aes_sbox_matches_fips197_for_all_inputs() {
    for input in 0..=u8::MAX {
        assert_eq!(sub_byte(input), REFERENCE_SBOX[input as usize], "input {input:#04x}");
    }
}
