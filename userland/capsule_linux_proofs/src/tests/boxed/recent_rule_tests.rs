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

//! The calls before a process's end are said for a nonzero exit and for an
//! end by SIGKILL, the end three Ctrl+C give a run stuck past any signal it
//! may catch; a clean exit and an end by another signal say no more here.

#[path = "../../../../capsule_linux/src/linux/recent_rule.rs"]
mod recent_rule;

use recent_rule::worth;

#[test]
fn a_nonzero_exit_and_a_kill_say_the_calls_before_them() {
    assert!(worth(1, 0) && worth(255, 0), "a nonzero exit");
    assert!(worth(0, 9), "SIGKILL, as three Ctrl+C end a run");
}

#[test]
fn a_clean_exit_or_another_signal_says_no_more() {
    assert!(!worth(0, 0), "a clean exit");
    for signo in [1, 2, 6, 11, 15] {
        assert!(!worth(0, signo), "signal {signo}");
    }
}
