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

//! PASSKEY and ANSWER: what pairing asks, and an answer that must fit it.

use super::fixture::{ask, board};
use crate::link::Answer;
use crate::nbtc::ops::*;
use crate::nbtc::who::Caller::Settings;
use crate::nbtc::Part;

fn answer(kind: u8, v: u32) -> Vec<u8> {
    [&[kind][..], &v.to_le_bytes()].concat()
}

#[test]
fn passkey_says_what_is_asked() {
    let mut b = board();
    let parts = [(Part::None, 0, 0), (Part::Show(123_456), 1, 123_456)];
    let parts = parts.into_iter().chain([(Part::Enter, 2, 0), (Part::Confirm(42), 3, 42)]);
    for (part, kind, v) in parts {
        b.ask(part);
        assert_eq!(ask(&mut b, Settings, OP_PASSKEY, &[], 0).1, answer(kind, v));
    }
}

#[test]
fn digits_typed_answer_a_passkey_once() {
    let mut b = board();
    b.ask(Part::Enter);
    assert_eq!(ask(&mut b, Settings, OP_LINK, &[], 0).1[1] & 0b10_0000, 0b10_0000, "it waits");
    assert_eq!(ask(&mut b, Settings, OP_ANSWER, &answer(PART_ENTER, 999_999), 0).0, E_OK);
    assert_eq!(b.part, Part::None);
    assert_eq!(ask(&mut b, Settings, OP_ANSWER, &answer(PART_ENTER, 1), 0).0, E_INVAL);
    assert_eq!(b.take_answer(), Some(Answer::Passkey(999_999)));
    assert_eq!(b.take_answer(), None, "the link acts on it once");
}

#[test]
fn an_answer_that_does_not_fit_is_refused() {
    let mut b = board();
    b.ask(Part::Enter);
    let wrong = [answer(PART_ENTER, 1_000_000), answer(PART_CONFIRM, 1), answer(PART_SHOW, 0)];
    for w in wrong.iter().chain([&vec![PART_ENTER, 0, 0]]) {
        assert_eq!(ask(&mut b, Settings, OP_ANSWER, w, 0).0, E_INVAL, "{w:?}");
    }
    b.ask(Part::Confirm(7));
    assert_eq!(ask(&mut b, Settings, OP_ANSWER, &answer(PART_CONFIRM, 2), 0).0, E_INVAL);
    b.ask(Part::Show(7));
    assert_eq!(ask(&mut b, Settings, OP_ANSWER, &answer(PART_NONE, 0), 0).0, E_INVAL);
    assert_eq!(b.take_answer(), None);
}
