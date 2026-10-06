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


use super::order::requeue;

#[test]
fn what_waited_runs_first_in_the_order_it_was_asked() {
    let waiting = vec!["linux.qwen-medium", "linux.qwen-coder-1.5b"];
    let newer = vec!["linux.qwen-small"];
    assert_eq!(
        requeue(waiting, newer),
        vec!["linux.qwen-medium", "linux.qwen-coder-1.5b", "linux.qwen-small"]
    );
}

#[test]
fn a_job_asked_again_while_it_waits_is_kept_once_in_its_first_place() {
    let waiting = vec!["linux.qwen-medium", "linux.qwen-coder-1.5b"];
    let newer = vec!["linux.qwen-coder-1.5b", "linux.qwen-medium", "linux.qwen-small"];
    assert_eq!(
        requeue(waiting, newer),
        vec!["linux.qwen-medium", "linux.qwen-coder-1.5b", "linux.qwen-small"]
    );
}

#[test]
fn nothing_waiting_leaves_what_was_asked_as_it_was() {
    assert_eq!(requeue(Vec::<u8>::new(), vec![3, 1, 2]), vec![3, 1, 2]);
    assert!(requeue(Vec::<u8>::new(), Vec::new()).is_empty());
}
