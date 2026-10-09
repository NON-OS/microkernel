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

use crate::authorize::CAP_NETWORK;
use crate::board::{read_answer, Board, PostError, ANSWER_LEN, SLOT_LEN};
use crate::report::{encode, Network, Stage};
use crate::report_tests::{ready_anyone, ready_nym};

#[test]
fn an_empty_board_answers_two_empty_slots() {
    let b = Board::new();
    let a = b.answer(5_000);
    assert_eq!(a.len(), ANSWER_LEN);
    assert_eq!(read_answer(&a), Some((None, None)));
}

#[test]
fn a_transports_post_is_kept_and_aged_on_the_boards_clock() {
    let mut b = Board::new();
    assert_eq!(b.post(b"net.anon", CAP_NETWORK, &encode(&ready_anyone()), 1_000), Ok(Network::Anyone));
    assert_eq!(b.post(b"net.nym", CAP_NETWORK, &encode(&ready_nym()), 4_000), Ok(Network::Nym));
    let (nym, anyone) = read_answer(&b.answer(10_000)).unwrap();
    assert_eq!(nym, Some((ready_nym(), 6_000)));
    assert_eq!(anyone, Some((ready_anyone(), 9_000)));
}

#[test]
fn a_post_from_anyone_else_changes_nothing() {
    let mut b = Board::new();
    b.post(b"net.anon", CAP_NETWORK, &encode(&ready_anyone()), 1_000).unwrap();
    let before = b.answer(2_000);
    let mut forged = ready_anyone();
    forged.stage = Stage::Failed;
    for (name, caps) in [
        (&b"app.browser"[..], u64::MAX),
        (b"net.anon", 0x18),
        (b"net.nym", CAP_NETWORK),
        (b"linux", CAP_NETWORK),
    ] {
        assert_eq!(b.post(name, caps, &encode(&forged), 1_500), Err(PostError::NotPermitted));
    }
    assert_eq!(b.answer(2_000), before);
}

#[test]
fn a_malformed_post_changes_nothing() {
    let mut b = Board::new();
    b.post(b"net.nym", CAP_NETWORK, &encode(&ready_nym()), 0).unwrap();
    let before = b.answer(100);
    let wire = encode(&ready_nym());
    assert_eq!(b.post(b"net.nym", CAP_NETWORK, &wire[..10], 50), Err(PostError::Malformed));
    assert_eq!(b.post(b"net.nym", CAP_NETWORK, &[], 50), Err(PostError::Malformed));
    let mut w = wire;
    w[1] = 9;
    assert_eq!(b.post(b"net.nym", CAP_NETWORK, &w, 50), Err(PostError::Malformed));
    assert_eq!(b.answer(100), before);
}

#[test]
fn a_cleared_slot_reads_empty_and_a_clock_step_back_reads_age_zero() {
    let mut b = Board::new();
    b.post(b"net.anon", CAP_NETWORK, &encode(&ready_anyone()), 9_000).unwrap();
    assert_eq!(b.latest(Network::Anyone, 1_000), Some((ready_anyone(), 0)));
    b.clear(Network::Anyone);
    assert_eq!(read_answer(&b.answer(10_000)), Some((None, None)));
}

#[test]
fn a_reader_refuses_answers_that_are_not_exactly_one() {
    let mut b = Board::new();
    b.post(b"net.nym", CAP_NETWORK, &encode(&ready_nym()), 0).unwrap();
    let a = b.answer(1);
    assert!(read_answer(&a[..ANSWER_LEN - 1]).is_none());
    // A slot flag other than 0 or 1.
    let mut x = a;
    x[0] = 2;
    assert!(read_answer(&x).is_none());
    // An empty slot with bytes in it.
    let mut x = a;
    x[SLOT_LEN + 3] = 1;
    assert!(read_answer(&x).is_none());
    // The Nym report placed in the Anyone slot.
    let mut x = [0u8; ANSWER_LEN];
    x[SLOT_LEN..].copy_from_slice(&a[..SLOT_LEN]);
    assert!(read_answer(&x).is_none());
}
