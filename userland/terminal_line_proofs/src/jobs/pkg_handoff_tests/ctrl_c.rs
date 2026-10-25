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

//! After Ctrl+C the worker's answer is discarded, whether it comes late or came already.

use super::{answer, Answer};
use crate::pkg_handoff::{Handoff, Look};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

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
