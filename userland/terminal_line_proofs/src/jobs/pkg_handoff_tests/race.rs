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

//! Ctrl+C racing the answer drops every answer exactly once.

use super::{answer, Answer};
use crate::pkg_handoff::{Handoff, Look};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

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
