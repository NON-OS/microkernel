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

//! A read the store never answered is not an empty disk: the restore asks
//! again. Only an answer that is not exactly a blob is a definite "none".

use super::answer::Answer;
use super::read_judge::{is_cleared, judge, Unread};

const START: usize = 4;

fn reply(data: &[u8]) -> Vec<u8> {
    let mut rx = vec![0u8; START];
    rx.extend_from_slice(data);
    rx
}

#[test]
fn a_silent_store_is_not_an_empty_disk() {
    let rx = reply(&[7u8; 8]);
    assert_eq!(judge::<8>(Answer::Silent, &rx, START), Err(Unread::Silent));
}

#[test]
fn a_refused_read_is_asked_again_never_taken_for_no_file() {
    let rx = reply(&[7u8; 8]);
    assert_eq!(judge::<8>(Answer::Refused(-5), &rx, START), Err(Unread::Silent));
}

#[test]
fn a_cleared_record_is_no_record_and_a_sealed_one_is_not_cleared() {
    assert!(is_cleared(&[0u8; 64]));
    let mut sealed = [0u8; 64];
    sealed[63] = 1;
    assert!(!is_cleared(&sealed));
}

#[test]
fn a_whole_blob_reads_back_exactly() {
    let blob: Vec<u8> = (0u8..8).collect();
    let rx = reply(&blob);
    assert_eq!(judge::<8>(Answer::Ok(rx.len()), &rx, START), Ok([0, 1, 2, 3, 4, 5, 6, 7]));
}

#[test]
fn a_short_or_long_read_is_not_a_blob() {
    let rx = reply(&[7u8; 8]);
    assert_eq!(judge::<8>(Answer::Ok(rx.len() - 1), &rx, START), Err(Unread::NotABlob));
    assert_eq!(judge::<4>(Answer::Ok(rx.len()), &rx, START), Err(Unread::NotABlob));
}

#[test]
fn a_length_past_the_buffer_is_not_a_blob() {
    let rx = reply(&[7u8; 8]);
    assert_eq!(judge::<8>(Answer::Ok(rx.len() + 4), &rx, START), Err(Unread::NotABlob));
    assert_eq!(judge::<8>(Answer::Ok(1), &rx, START), Err(Unread::NotABlob));
}

#[test]
fn only_an_ok_answer_has_a_length() {
    assert_eq!(Answer::Ok(3).len(), Some(3));
    assert_eq!(Answer::Refused(-2).len(), None);
    assert!(
        matches!(Answer::Refused(-28), Answer::Refused(code) if code == -28),
        "the code is kept"
    );
    assert_eq!(Answer::Silent.len(), None);
}
