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

//! A launch is followed from before its load to its end.

use super::settle::fresh;
use crate::launch::timing::{DEADLINE_MS, LOOK_GAP_MS};
use crate::launch::{Launch, Step};
use crate::state::Uptime;

#[test]
fn it_keeps_the_children_seen_before_the_load() {
    let l = fresh();
    assert_eq!(l.children, vec![3, 4]);
    assert_eq!(l.name, b"notes".to_vec());
    assert_eq!(l.elapsed_ms(Uptime(1_250)), 250);
}

/// Started in the shell's first seconds, when the wall clock still read its
/// error (-61) and never moved: on uptime the launch is due again a look gap
/// later and fails at its deadline, rather than waiting forever and holding
/// every later Launchpad launch behind it.
#[test]
fn a_launch_started_early_is_followed_to_its_end() {
    let mut l = Launch::new(b"notes", vec![], Uptime(40));
    assert!(l.due(Uptime(40)));
    assert!(l.due(Uptime(40 + LOOK_GAP_MS)));
    assert_eq!(l.step(Uptime(40 + DEADLINE_MS), None), Step::Failed);
}
