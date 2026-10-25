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

//! A late reply is an answer: the Terminal issues a load for 40 ms, the
//! installer answers it some 100 ms on, and the next load, 21 s later, finds
//! that reply. The pair was never silent, and is not said to be.

use super::silence::{Pair, Silence};

const W: u64 = 5000;
const TERM: Pair = Pair { caller: 44, port: 4112 };
const DROPS: &str = include_str!("../../../../src/syscall/microkernel/ipc/call/late_drops.rs");
const RECV: &str = include_str!("../../../../src/syscall/microkernel/ipc/recv_reply.rs");

#[test]
fn a_late_reply_found_ends_the_silence_of_its_call() {
    let mut s = Silence::new(W);
    assert_eq!(s.late(TERM, 163_000, 163_040, 11), [None, None]);
    s.replied(TERM.caller, 11);
    assert_eq!(s.late(TERM, 184_000, 184_040, 12), [None, None], "21 s apart, both answered");
    // Unfound, the two loads read as one silence of 21 s.
    let mut s = Silence::new(W);
    assert_eq!(s.late(TERM, 163_000, 163_040, 11), [None, None]);
    assert_eq!(s.late(TERM, 184_000, 184_040, 12), [Some(TERM), None]);
}

#[test]
fn only_the_reply_to_the_last_late_call_of_that_caller_ends_it() {
    let mut s = Silence::new(W);
    assert_eq!(s.late(TERM, 0, 20, 5), [None, None]);
    assert_eq!(s.late(TERM, 500, 520, 6), [None, None]);
    s.replied(TERM.caller, 5);
    s.replied(TERM.caller + 1, 6);
    assert_eq!(s.late(TERM, 5000, 5020, 7), [Some(TERM), None], "still silent since 0");
}

#[test]
fn the_reply_receive_says_whose_late_reply_it_dropped() {
    let drops: String = DROPS.split_whitespace().collect();
    let replied = "Relaxed);super::silence_table::replied(caller,correlation);";
    assert!(drops.contains(replied), "a late reply found ends its call's silence");
    let recv: String = RECV.split_whitespace().collect();
    assert!(recv.contains("super::call::note_drop(pid,msg.correlation,want);"));
}
