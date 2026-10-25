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

use super::rule::{group_of, others_alive};

const ALIVE: bool = true;
const ENDED: bool = false;

#[test]
fn a_group_of_zero_stands_for_itself() {
    assert_eq!(group_of(0, 41), 41);
    assert_eq!(group_of(40, 41), 40);
    assert_eq!(group_of(40, 40), 40);
}

#[test]
fn a_single_thread_gives_its_devices_back_as_it_ends() {
    // The only thread of its process: what it holds goes now, as before L32.
    assert!(!others_alive(40, 40, [(40, 40, ALIVE)]));
    assert!(!others_alive(40, 40, [(40, 0, ALIVE)]));
}

#[test]
fn a_worker_ending_leaves_the_devices_with_the_process() {
    let threads = [(40, 40, ALIVE), (41, 40, ALIVE)];
    assert!(others_alive(40, 41, threads), "the leader lives on");
    assert!(others_alive(40, 40, threads), "the worker lives on after the leader");
}

#[test]
fn the_last_live_thread_gives_them_back() {
    // The leader is a zombie; the worker ending now is the last.
    assert!(!others_alive(40, 41, [(40, 40, ENDED), (41, 40, ALIVE)]));
}

#[test]
fn another_process_never_keeps_them() {
    let threads = [(40, 40, ALIVE), (50, 50, ALIVE), (51, 50, ALIVE), (52, 0, ALIVE)];
    assert!(!others_alive(40, 40, threads));
    // A thread whose own group is 0 is its own process, never 40's.
    assert!(!others_alive(40, 40, [(40, 40, ALIVE), (52, 0, ALIVE)]));
}

#[test]
fn two_threads_ending_together_leave_it_to_the_later_finalize() {
    // Both still alive at teardown: neither releases.
    let both = [(40, 40, ALIVE), (41, 40, ALIVE)];
    assert!(others_alive(40, 40, both) && others_alive(40, 41, both));
    // At finalize the other is a zombie: whichever runs it releases.
    assert!(!others_alive(40, 40, [(40, 40, ENDED), (41, 40, ENDED)]));
}
