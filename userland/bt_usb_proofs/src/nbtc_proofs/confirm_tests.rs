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

//! ANSWER for Numeric Comparison, a cancel, and a part HID readiness ends.

use super::fixture::{ask, board};
use crate::link::Answer;
use crate::nbtc::ops::*;
use crate::nbtc::who::Caller::Settings;
use crate::nbtc::Part;

fn answer(kind: u8, v: u32) -> Vec<u8> {
    [&[kind][..], &v.to_le_bytes()].concat()
}

#[test]
fn a_number_is_confirmed_or_declined_and_any_part_cancelled() {
    let mut b = board();
    b.ask(Part::Confirm(7));
    ask(&mut b, Settings, OP_ANSWER, &answer(PART_CONFIRM, 0), 0);
    assert_eq!(b.take_answer(), Some(Answer::Same(false)));
    b.ask(Part::Enter);
    ask(&mut b, Settings, OP_ANSWER, &answer(PART_NONE, 0), 0);
    assert_eq!(b.take_answer(), Some(Answer::Cancel));
    b.ask(Part::Confirm(7));
    b.ready(true, false, true);
    assert_eq!((b.part, b.take_answer()), (Part::None, None), "HID ready: nothing is asked");
}
