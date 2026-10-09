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

use super::bitmap::{clear_run, take_run};
use super::run_taken::run_taken;

#[test]
fn a_run_freed_twice_is_refused_and_leaves_the_next_grant_alone() {
    let mut used = [0u64; 1];
    let first = take_run(&mut used, 64, 4).unwrap();
    assert!(run_taken(&used, first, 4));
    clear_run(&mut used, first, 4);
    let second = take_run(&mut used, 64, 2).unwrap();
    assert_eq!(second, first);
    assert!(!run_taken(&used, first, 4));
    assert!(run_taken(&used, second, 2));
}

#[test]
fn a_run_reaching_over_a_free_page_or_empty_is_refused() {
    let mut used = [0u64; 1];
    take_run(&mut used, 64, 3).unwrap();
    assert!(!run_taken(&used, 1, 3));
    assert!(!run_taken(&used, 0, 0));
    assert!(run_taken(&used, 1, 2));
}
