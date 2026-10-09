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

//! `pkg` asks the installer from a worker thread and the window thread picks
//! the answer up on its tick: one request at a time, the answer taken once,
//! and after Ctrl+C the worker's late answer discarded rather than handed to
//! a later request. The worker here is a real host thread.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use crate::pkg_handoff::{Handoff, Look};

/// An answer that counts how often it is dropped.
#[derive(Debug)]
struct Answer {
    n: u32,
    drops: Arc<AtomicUsize>,
}

impl PartialEq for Answer {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Drop for Answer {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

fn answer(n: u32, drops: &Arc<AtomicUsize>) -> Answer {
    Answer { n, drops: drops.clone() }
}

fn wait_ready<T>(slot: &Handoff<T>) -> T {
    loop {
        match slot.poll() {
            Look::Ready(v) => return v,
            Look::Waiting => thread::yield_now(),
            Look::Idle => panic!("the slot went idle with no answer"),
        }
    }
}

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
fn after_ctrl_c_the_late_answer_is_discarded_and_never_handed_on() {
    let drops = Arc::new(AtomicUsize::new(0));
    let slot: Handoff<Answer> = Handoff::new();
    assert!(slot.claim());
    slot.stop_waiting();
    // The worker is still on it: the slot stays taken.
    assert_eq!(slot.poll(), Look::Waiting);
    assert!(!slot.claim(), "a new request waits for the dropped worker to finish");
    slot.put(answer(1, &drops));
    assert_eq!(drops.load(Ordering::SeqCst), 1, "the late answer is dropped where it lands");
    assert_eq!(slot.poll(), Look::Idle, "and nobody is handed it");
    assert!(slot.claim());
    slot.put(answer(2, &drops));
    assert_eq!(slot.poll(), Look::Ready(answer(2, &drops)));
}

#[test]
fn ctrl_c_after_the_answer_came_discards_it_at_once() {
    let drops = Arc::new(AtomicUsize::new(0));
    let slot: Handoff<Answer> = Handoff::new();
    assert!(slot.claim());
    slot.put(answer(3, &drops));
    slot.stop_waiting();
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(slot.poll(), Look::Idle);
    assert!(slot.claim());
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

#[test]
fn ctrl_c_racing_the_answer_drops_every_answer_exactly_once() {
    let drops = Arc::new(AtomicUsize::new(0));
    let slot: &'static Handoff<Answer> = Box::leak(Box::new(Handoff::new()));
    let rounds = 2_000u32;
    let mut taken = 0usize;
    for n in 0..rounds {
        assert!(slot.claim(), "round {n}: the slot is free");
        let d = drops.clone();
        let worker = thread::spawn(move || slot.put(answer(n, &d)));
        if n % 2 == 0 {
            slot.stop_waiting();
        }
        worker.join().unwrap();
        match slot.poll() {
            Look::Ready(a) => {
                assert_eq!(a.n, n, "only this round's answer");
                assert!(n % 2 == 1, "a dropped wait is never answered");
                taken += 1;
            }
            Look::Idle => assert!(n % 2 == 0, "a kept wait gets its answer"),
            Look::Waiting => panic!("round {n}: the worker has finished"),
        }
    }
    assert_eq!(taken, rounds as usize / 2);
    assert_eq!(drops.load(Ordering::SeqCst), rounds as usize, "each answer dropped once");
}
