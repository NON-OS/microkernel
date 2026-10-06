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

//! Proofs that a batched answer from net.nym is read back into the messages
//! it carried: several in one answer, one split across answers, and that a
//! piece which cannot be placed, or an answer that stops making sense, is
//! dropped rather than handed on as a message it is not.

use crate::batch::{Joiner, FLAG_CONT, FLAG_MORE, MESSAGE_MAX};

fn record(flags: u8, bytes: &[u8]) -> Vec<u8> {
    let mut out = vec![flags];
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(bytes);
    out
}

#[test]
fn several_whole_messages_in_one_answer_come_out_in_order() {
    let mut answer = record(0, b"first");
    answer.extend(record(0, b""));
    answer.extend(record(0, b"third"));
    let got = Joiner::new().feed(&answer);
    assert_eq!(got.messages, vec![b"first".to_vec(), Vec::new(), b"third".to_vec()]);
    assert_eq!(got.dropped, 0);
    assert!(!got.malformed);
}

#[test]
fn a_message_split_across_answers_comes_out_whole() {
    let mut joiner = Joiner::new();
    let first = joiner.feed(&record(FLAG_MORE, b"front "));
    assert!(first.messages.is_empty(), "the front alone is not a message");
    let middle = joiner.feed(&record(FLAG_CONT | FLAG_MORE, b"middle "));
    assert!(middle.messages.is_empty());
    let mut last = record(FLAG_CONT, b"back");
    last.extend(record(0, b"next"));
    let got = joiner.feed(&last);
    assert_eq!(got.messages, vec![b"front middle back".to_vec(), b"next".to_vec()]);
    assert_eq!(got.dropped, 0);
}

#[test]
fn a_continuation_with_nothing_begun_is_dropped() {
    let got = Joiner::new().feed(&record(FLAG_CONT, b"orphan"));
    assert!(got.messages.is_empty(), "a tail is never read as a whole message");
    assert_eq!(got.dropped, 1);
}

#[test]
fn a_new_message_abandons_one_left_unfinished() {
    let mut joiner = Joiner::new();
    joiner.feed(&record(FLAG_MORE, b"lost its tail"));
    let got = joiner.feed(&record(0, b"fresh"));
    assert_eq!(got.messages, vec![b"fresh".to_vec()]);
    assert_eq!(got.dropped, 1, "the unfinished one is dropped, not glued to the next");
}

#[test]
fn a_message_growing_past_the_bound_is_abandoned() {
    let mut joiner = Joiner::new();
    let piece = vec![0u8; 200 * 1024];
    joiner.feed(&record(FLAG_MORE, &piece));
    joiner.feed(&record(FLAG_CONT | FLAG_MORE, &piece));
    let got = joiner.feed(&record(FLAG_CONT | FLAG_MORE, &piece));
    assert!(3 * piece.len() > MESSAGE_MAX);
    assert_eq!(got.dropped, 1);
    let after = joiner.feed(&record(FLAG_CONT, b"tail"));
    assert!(after.messages.is_empty(), "nothing is left to finish");
}

#[test]
fn a_record_claiming_more_than_the_answer_holds_stops_the_read() {
    let mut answer = record(0, b"good");
    answer.extend_from_slice(&[0, 0xff, 0xff, 0xff, 0x7f, 1, 2, 3]);
    let got = Joiner::new().feed(&answer);
    assert_eq!(got.messages, vec![b"good".to_vec()], "what came before is kept");
    assert!(got.malformed);
}

#[test]
fn a_truncated_header_and_unknown_flags_are_malformed() {
    let got = Joiner::new().feed(&[0, 1, 0]);
    assert!(got.malformed && got.messages.is_empty());
    let got = Joiner::new().feed(&record(0x80, b"x"));
    assert!(got.malformed && got.messages.is_empty());
}

#[test]
fn a_malformed_answer_abandons_a_message_part_way() {
    let mut joiner = Joiner::new();
    joiner.feed(&record(FLAG_MORE, b"front"));
    let got = joiner.feed(&[9, 9]);
    assert!(got.malformed);
    assert_eq!(got.dropped, 1);
    let after = joiner.feed(&record(FLAG_CONT, b"back"));
    assert!(after.messages.is_empty(), "the rest cannot be trusted to belong to it");
}

#[test]
fn arbitrary_answers_never_panic() {
    let mut state = 0x1234_5678u64;
    let mut joiner = Joiner::new();
    for _ in 0..20_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let len = (state >> 58) as usize;
        let bytes: Vec<u8> = (0..len)
            .map(|i| {
                (state.rotate_left(i as u32 * 7) >> 11) as u8 & if i % 5 == 1 { 3 } else { 0xff }
            })
            .collect();
        let got = joiner.feed(&bytes);
        assert!(got.messages.iter().all(|m| m.len() <= MESSAGE_MAX));
    }
}
