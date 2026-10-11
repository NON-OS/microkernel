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

//! The generated vendor table: every row is a canonical policy for its own
//! program, with no listing or release, as the market checks it.

use crate::dests::{check, dests_hash};
use crate::vendor::{by_program, TABLE};
use crate::{decode, unlisted};

#[test]
fn every_row_is_its_programs_canonical_vendor_policy() {
    for row in TABLE {
        let p = decode(&row.policy).expect(row.store_path);
        assert_eq!(p.encode(), row.policy, "{}", row.store_path);
        assert_eq!(p.program_blake3, row.program_blake3, "{}", row.store_path);
        assert_eq!((p.listing_id, p.release_id), ([0; 32], [0; 32]), "{}", row.store_path);
        assert_eq!(by_program(&row.program_blake3).map(|r| r.policy), Some(row.policy));
        assert!(row.store_path.starts_with("/bin/"));
    }
}

#[test]
fn a_program_the_vendor_does_not_ship_has_no_row() {
    let stranger = [0xee; 32];
    assert!(TABLE.iter().all(|r| r.program_blake3 != stranger));
    assert_eq!(by_program(&stranger), None);
    assert_eq!(unlisted(stranger).program_blake3, stranger);
}

#[test]
fn every_rows_list_is_the_one_its_policy_names() {
    for row in TABLE {
        let hash = decode(&row.policy).expect(row.store_path).net_dests_hash;
        match row.dests {
            [] => assert_eq!(hash, [0; 32], "{}", row.store_path),
            list => {
                assert!(check(list).is_ok(), "{}", row.store_path);
                assert_eq!(hash, dests_hash(list), "{}", row.store_path);
            }
        }
    }
}
