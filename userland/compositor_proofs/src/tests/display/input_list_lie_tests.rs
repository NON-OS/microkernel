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

//! A driver reading that lies about its own length, cut short anywhere or
//! counting values it does not carry, offers nothing; a whole one offers
//! what it lists, eight at most.

use super::input_list_tests::reading;
use crate::display::input_list::offered;

#[test]
fn a_reading_that_lies_about_its_length_offers_nothing() {
    let mut s = 0x1A9B_u64;
    let (mut taken, mut refused) = (0, 0);
    for _ in 0..100_000 {
        s = crate::tests::requests::rng::Rng::new(s).next();
        let n = (s % 12) as usize;
        let mut r = reading(if s >> 60 == 0 { -5 } else { 0 }, (s >> 8) as u16, &vec![7; n]);
        r[8] = (s >> 20) as u8 % 14;
        r.truncate((s >> 32) as usize % (r.len() + 1));
        let said = r.get(8).map(|&k| k as usize);
        let whole =
            r.len() >= 9 && r[..4] == [0; 4] && said.is_some_and(|k| k > 0 && r.len() >= 9 + k);
        let got = offered(5, &r);
        assert_eq!(got.is_some(), whole, "{r:02X?}");
        if let Some(o) = got {
            assert_eq!(o.n as usize, said.unwrap().min(8));
        }
        (taken, refused) = (taken + whole as usize, refused + !whole as usize);
    }
    assert!(taken >= 5_000 && refused >= 50_000, "{taken} {refused}");
}
