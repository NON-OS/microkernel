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

//! The Wi-Fi core's own S-box test, kept now that the core takes its AES
//! from nonos_aes: the computed S-box is FIPS 197's for every byte, and the
//! computed inverse undoes it, sixteen bytes at a time as a state is.

use super::sbox_table::REFERENCE_SBOX;
use crate::inv_sub_byte::inv_sub_byte;
use crate::sub_byte::sub_byte;

#[test]
fn the_computed_s_box_is_fips_197s_for_every_byte() {
    for base in (0..=255u8).step_by(16) {
        let mut s: [u8; 16] = core::array::from_fn(|j| base + j as u8);
        s = s.map(sub_byte);
        for (j, got) in s.iter().enumerate() {
            assert_eq!(*got, REFERENCE_SBOX[base as usize + j], "{:#04x}", base as usize + j);
        }
        s = s.map(inv_sub_byte);
        assert_eq!(s, core::array::from_fn(|j| base + j as u8), "the inverse undoes it");
    }
}
