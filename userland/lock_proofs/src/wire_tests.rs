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

//! The keyring's lock operations on the wire.

use nonos_lock::wire::*;

#[test]
fn a_set_request_reads_back() {
    let s = set_request(7, 42, b"old pass", b"new passphrase").unwrap();
    let b = s.as_slice();
    assert_eq!(&b[0..4], &7u32.to_le_bytes());
    assert_eq!(&b[4..6], &OP_LOCK_SET.to_le_bytes());
    assert_eq!(parse_set(&b[HDR_LEN..]), Some((42, &b"old pass"[..], &b"new passphrase"[..])));
    let first = set_request(1, 42, b"", b"new passphrase").unwrap();
    assert_eq!(
        parse_set(&first.as_slice()[HDR_LEN..]),
        Some((42, &b""[..], &b"new passphrase"[..]))
    );
}

#[test]
fn a_check_load_and_state_request_read_back() {
    let c = check_request(1, 9, b"secret words").unwrap();
    assert_eq!(&c.as_slice()[4..6], &OP_LOCK_CHECK.to_le_bytes());
    assert_eq!(parse_check(&c.as_slice()[HDR_LEN..]), Some((9, &b"secret words"[..])));
    let l = load_request(2, 9, &[1, 2, 3]);
    assert_eq!(parse_load(&l.as_slice()[HDR_LEN..]), Some((9, &[1u8, 2, 3][..])));
    let s = state_request(3, 9);
    assert_eq!(parse_state(&s.as_slice()[HDR_LEN..]), Some(9));
}

#[test]
fn bodies_of_the_wrong_length_are_refused() {
    let c = check_request(1, 9, b"secret words").unwrap();
    let body = &c.as_slice()[HDR_LEN..];
    assert_eq!(parse_check(&body[..body.len() - 1]), None);
    let mut longer = body.to_vec();
    longer.push(0);
    assert_eq!(parse_check(&longer), None);
    let s = set_request(1, 9, b"a", b"b").unwrap();
    let body = &s.as_slice()[HDR_LEN..];
    assert_eq!(parse_set(&body[..body.len() - 1]), None);
    assert_eq!(parse_set(&body[..5]), None);
    assert_eq!(parse_state(&[1, 2, 3]), None);
    assert_eq!(parse_state(&[1, 2, 3, 4, 5]), None);
    assert_eq!(parse_load(&[1, 2]), None);
}

#[test]
fn a_passphrase_too_long_for_the_wire_builds_nothing() {
    let huge = vec![b'a'; 70_000];
    assert!(check_request(1, 1, &huge).is_none());
    assert!(set_request(1, 1, &huge, b"x").is_none());
}

#[test]
fn statuses_map_to_answers() {
    let count = Count { failures: 4, wait_ms: 2000 };
    let mut wrong = count.encode().to_vec();
    wrong.extend_from_slice(&[1, 2]);
    assert_eq!(answer(OK, &[8, 9]), Answer::Done(&[8, 9]));
    assert_eq!(answer(E_WRONG, &wrong), Answer::Wrong(count, &[1, 2]));
    assert_eq!(answer(E_WAIT, &count.encode()), Answer::Wait(count));
    assert_eq!(answer(E_NOENT, &[]), Answer::NoRecord);
    assert_eq!(answer(E_EXIST, &[]), Answer::Held);
    assert_eq!(answer(E_ACCES, &[]), Answer::Refused(E_ACCES));
    assert_eq!(answer(E_WRONG, &[1]), Answer::Refused(E_INVAL));
    assert_eq!(answer(E_WAIT, &[]), Answer::Refused(E_INVAL));
}

#[test]
fn a_reply_must_answer_its_own_request() {
    let mut r = 5u32.to_le_bytes().to_vec();
    r.extend_from_slice(&E_WRONG.to_le_bytes());
    r.extend_from_slice(&[1, 2]);
    assert_eq!(reply(&r, 5), Some((E_WRONG, &[1u8, 2][..])));
    assert_eq!(reply(&r, 6), None);
    assert_eq!(reply(&r[..7], 5), None);
}

#[test]
fn the_standing_reads_back() {
    let c = Count { failures: 2, wait_ms: 0 };
    let body = standing(true, c, &[4, 4]);
    assert_eq!(parse_standing(&body), Some((true, c, &[4u8, 4][..])));
    assert_eq!(
        parse_standing(&standing(false, Count::default(), &[])),
        Some((false, Count::default(), &[][..]))
    );
    assert_eq!(parse_standing(&[1, 0, 0]), None);
}

#[test]
fn the_ops_follow_the_keyrings_own() {
    assert_eq!(OPS, [32, 33, 34, 35]);
}
