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

//! The parked table the seal's answers come through.

use crate::foreign::frame::ForeignFrame;
use crate::foreign::registry::{insert, remove};
use crate::foreign::trap_table::{park, parked_nr, take_answer, Answer, PARKED};

#[test]
fn a_guest_stays_parked_until_its_answer_is_taken_once() {
    let (sup, guest) = (9101, 9102);
    assert!(insert(guest, sup));
    assert!(park(ForeignFrame::new(guest, 39, [0; 6], 0)));
    assert_eq!(parked_nr(guest), Some(39));
    assert!(take_answer(guest).is_none(), "no answer yet");
    {
        let mut parked = PARKED.lock();
        let p = parked.iter_mut().find(|p| p.frame.pid == guest).expect("parked");
        assert!(!p.claimed, "no wait has taken it");
        p.answer = Some(Answer::Value(4));
    }
    assert_eq!(parked_nr(guest), None, "answered");
    assert!(matches!(take_answer(guest), Some(Answer::Value(4))));
    assert!(take_answer(guest).is_none(), "taken once");
    remove(guest);
}

#[test]
fn a_frame_whose_supervisor_has_gone_is_not_parked() {
    let pid = 9103;
    assert!(!park(ForeignFrame::new(pid, 39, [0; 6], 0)), "not foreign: refused");
    assert!(!PARKED.lock().iter().any(|p| p.frame.pid == pid), "nothing left behind");
}
