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

//! The names the wipe clears: every byte and the length, whatever was there.

use crate::names_clear::clear;
use crate::sweep::Rng;

#[test]
fn a_name_is_cleared_to_its_last_byte_and_its_length() {
    let mut name = [0u8; 64];
    name[..10].copy_from_slice(b"zqwalkhost");
    name[63] = 0xff;
    let mut len = 10;
    clear(&mut name, &mut len);
    assert_eq!((name, len), ([0u8; 64], 0));
}

#[test]
fn random_names_of_every_length_are_cleared() {
    let mut r = Rng::new(0x5eed_0007);
    for _ in 0..5_000 {
        let mut name = [0u8; 64];
        r.fill(&mut name);
        let mut len = r.below(65) as usize;
        clear(&mut name, &mut len);
        assert!(name.iter().all(|b| *b == 0) && len == 0);
    }
}
