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

//! Reading a proxy's answer: the marker, the bytes, and every way an
//! answer can be wrong, refused before any of it is used.

use crate::answer::{
    arrived, decode, Answer, Malformed, ANSWER_BUF, ANSWER_MAX, CLOSED, LOST, OPEN,
};

#[test]
fn an_answer_is_a_marker_and_its_bytes() {
    assert_eq!(decode(&[OPEN]), Ok(Answer { closed: false, bytes: &[] }));
    assert_eq!(decode(&[OPEN, 1, 2]), Ok(Answer { closed: false, bytes: &[1, 2] }));
    /* The last bytes of a stream come with its close. */
    assert_eq!(decode(&[CLOSED, 9]), Ok(Answer { closed: true, bytes: &[9] }));
    assert_eq!(decode(&[CLOSED]), Ok(Answer { closed: true, bytes: &[] }));
}

#[test]
fn a_wrong_marker_or_none_is_refused() {
    assert_eq!(decode(&[]), Err(Malformed::Empty));
    for m in 3..=u8::MAX {
        assert_eq!(decode(&[m, 0, 0]), Err(Malformed::Marker(m)));
    }
}

#[test]
fn a_lost_conversation_is_told_apart_from_a_close() {
    assert_eq!(decode(&[LOST]), Err(Malformed::Lost));
    assert_ne!(LOST, CLOSED);
}

#[test]
fn an_answer_longer_than_any_proxy_builds_is_refused() {
    let longest = vec![OPEN; ANSWER_MAX];
    assert_eq!(decode(&longest).map(|a| a.bytes.len()), Ok(ANSWER_MAX - 1));
    let over = vec![OPEN; ANSWER_MAX + 1];
    assert_eq!(decode(&over), Err(Malformed::Oversized(ANSWER_MAX + 1)));
}

/* The buffer is larger than any answer, so a cut one shows. */
const _: () = assert!(ANSWER_BUF > ANSWER_MAX);

#[test]
fn a_length_past_the_buffer_or_negative_is_no_answer() {
    assert_eq!(arrived(-1, 64), None);
    assert_eq!(arrived(i64::MIN, 64), None);
    assert_eq!(arrived(65, 64), None);
    assert_eq!(arrived(i64::MAX, ANSWER_BUF), None);
    assert_eq!(arrived(64, 64), Some(64));
    assert_eq!(arrived(0, 64), Some(0));
}
