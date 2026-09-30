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
//! The policy indices setup offers are the ones a driver can resolve.

use nonos_keymap::{Layout, POLICY_LAYOUTS};

#[test]
fn every_offered_index_names_a_distinct_layout() {
    let mut seen = [false; Layout::COUNT as usize];
    for index in POLICY_LAYOUTS {
        let layout = Layout::from_policy(index).expect("offered layout has a table");
        assert!(!seen[layout.index() as usize], "two indices name one layout");
        seen[layout.index() as usize] = true;
    }
    assert!(seen.iter().all(|s| *s));
}

#[test]
fn layouts_without_tables_map_to_nothing() {
    for index in [1u8, 7, 8, 9, 200] {
        assert_eq!(Layout::from_policy(index), None);
    }
}
