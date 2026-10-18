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

//! The clock as the tests run it (`crate::clock`). A wait whose questions
//! are spent ends only once each part running on this thread's windows has
//! taken a turn since the last question, and a wait begins only between two
//! of a part's answers. A wait used to count questions and nothing else, so
//! a part the scheduler held back for longer than one (some 150 ms of wall
//! time) read as a part that never answered, and a reset could land midway
//! through an answer the part then finished in the restarted queue.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use crate::clock::expired_after;
use crate::parts;

#[test]
fn a_wait_with_no_part_running_ends_when_its_questions_are_spent() {
    let mut expired = expired_after(3);
    assert_eq!([expired(), expired(), expired()], [false, false, true]);
}

#[test]
fn a_spent_wait_ends_only_once_the_part_has_had_a_turn_since_its_last_question() {
    let link = parts::attach();
    let mut expired = expired_after(3);
    let asked: Vec<bool> = (0..6).map(|_| expired()).collect();
    assert_eq!(asked, [false; 6], "a wait ended while its part was held back");
    link.turned();
    assert!(expired(), "the wait outlived the part's turn");
}

#[test]
fn a_wait_begins_only_between_two_of_a_parts_answers() {
    let link = parts::attach();
    let (answered, (told, midway)) = (Arc::new(AtomicBool::new(false)), mpsc::channel());
    let (part, done) = (Arc::clone(&link), Arc::clone(&answered));
    let answering = thread::spawn(move || {
        let _answering = part.step();
        told.send(()).expect("the test is waiting");
        thread::sleep(Duration::from_millis(100));
        done.store(true, Ordering::Release);
    });
    midway.recv().expect("the part began an answer");
    drop(expired_after(1));
    assert!(answered.load(Ordering::Acquire), "a wait began midway through an answer");
    answering.join().expect("the part");
}
