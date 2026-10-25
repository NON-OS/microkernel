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

//! Every id range in a spawned driver's row runs low to high.

use super::scope::{row, Match};
use super::spawned::spawned_drivers;

#[test]
fn every_id_range_is_a_pair_low_to_high() {
    for d in spawned_drivers() {
        for m in row(&d).unwrap_or(&[]) {
            if let Match::Ids(_, ids) = m {
                assert!(!ids.is_empty() && ids.len() % 2 == 0, "{d}: a range without its end");
                assert!(ids.chunks(2).all(|r| r[0] <= r[1]), "{d}: a range high to low");
            }
        }
    }
}
