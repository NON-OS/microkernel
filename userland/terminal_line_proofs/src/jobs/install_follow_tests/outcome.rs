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

//! How a followed load ends: the loaded program, a failed load, or the deadline.

use super::fresh;
use crate::install_follow::{Step, DEADLINE_MS};

#[test]
fn a_new_child_is_the_loaded_program() {
    let mut f = fresh();
    assert_eq!(f.step(10_050, Some(31)), Step::Loaded(31));
    f.probed(10_600, true);
    assert_eq!(f.step(10_600, Some(31)), Step::Loaded(31));
}

#[test]
fn a_finished_installer_with_no_child_failed_the_load() {
    let mut f = fresh();
    f.probed(10_500, true);
    assert_eq!(f.step(10_500, None), Step::Failed);
}

#[test]
fn the_deadline_ends_a_load_that_never_settles() {
    let f = fresh();
    assert_eq!(f.step(10_000 + DEADLINE_MS, None), Step::TimedOut);
}
