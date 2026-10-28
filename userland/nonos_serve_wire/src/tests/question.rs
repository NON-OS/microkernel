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

//! A question carries only what its op may (P.9).

use super::frame;
use crate::*;

#[test]
fn a_question_carries_only_what_its_op_may() {
    assert!(request(&frame(OPEN, 0, 0, b"")).is_ok());
    assert_eq!(request(&frame(OPEN, 0, 0, b"x")), Err(Refused::Body));
    assert_eq!(request(&frame(CLOSE, 0, 3, b"x")), Err(Refused::Body));
    assert_eq!(request(&frame(OPEN, EBUSY, 0, b"")), Err(Refused::Body), "a question has status 0");
    assert_eq!(took_of(&took(65_532)), Some(65_532));
    assert_eq!(took_of(b"abc"), None);
}

#[test]
fn write_and_read_carry_their_number_and_stay_in_bounds() {
    let w = [&write_head(7)[..], b"abc"].concat();
    assert!(request(&frame(WRITE, 0, 3, &w)).is_ok());
    assert_eq!(written(&w), Some((7, &b"abc"[..])));
    assert_eq!(
        request(&frame(WRITE, 0, 3, &write_head(7))),
        Err(Refused::Body),
        "a number, no bytes"
    );
    assert_eq!(request(&frame(WRITE, 0, 3, b"")), Err(Refused::Body));
    assert!(request(&frame(READ, 0, 3, &read_ask(9, 4096, 5_000))).is_ok());
    assert_eq!(read_asked(&read_ask(9, 4096, 5_000)), Some((9, 4096, 5_000)));
    assert_eq!(request(&frame(READ, 0, 3, &read_ask(9, 4096, 5_001))), Err(Refused::Body));
    assert_eq!(request(&frame(READ, 0, 3, &read_ask(9, 0, 10))), Err(Refused::Body));
    let past = read_ask(9, MAX_BODY as u32 + 1, 10);
    assert_eq!(request(&frame(READ, 0, 3, &past)), Err(Refused::Body));
    assert_eq!(request(&frame(READ, 0, 3, &read_ask(9, 1, 1)[..8])), Err(Refused::Body));
}
