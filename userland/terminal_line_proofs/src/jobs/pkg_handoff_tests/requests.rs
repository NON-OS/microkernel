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

//! One request at a time, its answer taken once, and an unstarted claim given back.

use super::wait_ready;
use crate::pkg_handoff::{Handoff, Look};
use std::thread;

#[test]
fn a_second_request_is_refused_while_one_is_out() {
    let slot: Handoff<u32> = Handoff::new();
    assert!(slot.claim());
    assert!(!slot.claim(), "one worker at a time");
    slot.put(7);
    assert!(!slot.claim(), "an answer not yet taken still holds the slot");
    assert_eq!(slot.poll(), Look::Ready(7));
    assert!(slot.claim(), "taken, the slot is free for the next request");
}

#[test]
fn the_window_waits_then_takes_the_workers_answer_once() {
    let slot: &'static Handoff<u32> = Box::leak(Box::new(Handoff::new()));
    assert_eq!(slot.poll(), Look::Idle);
    assert!(slot.claim());
    assert_eq!(slot.poll(), Look::Waiting);
    let worker = thread::spawn(move || slot.put(42));
    assert_eq!(wait_ready(slot), 42);
    worker.join().unwrap();
    assert_eq!(slot.poll(), Look::Idle, "an answer is handed over once");
}

#[test]
fn a_claim_whose_worker_never_started_is_given_back() {
    let slot: Handoff<u32> = Handoff::new();
    assert!(slot.claim());
    slot.release();
    assert_eq!(slot.poll(), Look::Idle);
    assert!(slot.claim());
    slot.put(5);
    // A release does not take an answer away from the window.
    slot.release();
    assert_eq!(slot.poll(), Look::Ready(5));
}
