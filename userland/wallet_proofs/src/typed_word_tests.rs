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

//! A word must be the canonical encoding of its field's type.

use crate::server::typed::field::{well_formed, Kind};

#[test]
fn every_word_must_fit_its_type() {
    let mut w = [0u8; 32];
    w[31] = 1;
    for k in [Kind::Bytes32, Kind::Address, Kind::Uint(8), Kind::Uint(64), Kind::Uint(256)] {
        assert!(well_formed(k, &w), "{k:?}");
    }
    w[11] = 1;
    assert!(!well_formed(Kind::Address, &w) && well_formed(Kind::Uint(168), &w));
    w[0] = 1;
    assert!(well_formed(Kind::Bytes32, &w) && well_formed(Kind::Uint(256), &w));
    assert!(!well_formed(Kind::Uint(248), &w));
    for odd in [0, 7, 9, 257, 512] {
        assert!(!well_formed(Kind::Uint(odd), &[0; 32]), "{odd}");
    }
}
