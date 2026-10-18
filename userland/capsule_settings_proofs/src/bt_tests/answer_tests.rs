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

//! The passkey keys: digits and Enter answer a passkey the device shows, Y
//! and N a number both sides show, and none of them anything else.

use super::fixture::{answered, asked};
use crate::bluetooth::panel::key::BtKey;
use crate::bluetooth::panel::{act, Request, Step};
use nonos_bt_proto::ops::{OP_ANSWER, SECURING};
use nonos_bt_proto::{answer_body, Asked, Link, Reply};

fn answer(r: Reply) -> Step {
    Step::Ask(Request::new(OP_ANSWER, &answer_body(r).unwrap()))
}

#[test]
fn six_digits_and_enter_answer_the_device_shown_passkey() {
    let mut p = answered(SECURING, Link::WAITING);
    asked(&mut p, Asked::Enter);
    for d in *b"04321" {
        assert_eq!(act(&mut p, BtKey::Digit(d)), Step::Shown);
    }
    assert_eq!(act(&mut p, BtKey::Enter), Step::Shown, "five digits are not a passkey");
    assert_eq!(p.notice, Some("Type all six digits the device shows"));
    assert_eq!(act(&mut p, BtKey::Digit(b'9')), Step::Shown);
    assert_eq!(act(&mut p, BtKey::Digit(b'8')), Step::Nothing, "a seventh is not taken");
    assert_eq!(act(&mut p, BtKey::Erase), Step::Shown);
    assert_eq!(act(&mut p, BtKey::Digit(b'0')), Step::Shown);
    assert_eq!(p.digits.as_bytes(), b"043210");
    assert_eq!(act(&mut p, BtKey::Enter), answer(Reply::Digits(43_210)), "the leading zero kept");
    assert_eq!(act(&mut p, BtKey::No), answer(Reply::Decline));
    assert_eq!(act(&mut p, BtKey::Yes), Step::Nothing, "no number to confirm");
}

#[test]
fn y_and_n_answer_only_a_number_to_confirm() {
    let mut p = answered(SECURING, Link::WAITING);
    asked(&mut p, Asked::Confirm(123_456));
    assert_eq!(act(&mut p, BtKey::Yes), answer(Reply::Same(true)));
    assert_eq!(act(&mut p, BtKey::No), answer(Reply::Same(false)));
    assert_eq!(act(&mut p, BtKey::Digit(b'1')), Step::Nothing);
    assert_eq!(act(&mut p, BtKey::Enter), Step::Nothing);
    for nothing_to_answer in [Asked::None, Asked::Show(5)] {
        asked(&mut p, nothing_to_answer);
        for key in [BtKey::Yes, BtKey::No, BtKey::Enter, BtKey::Digit(b'5'), BtKey::Erase] {
            assert_eq!(act(&mut p, key), Step::Nothing, "{key:?}");
        }
    }
}
