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
use super::rig::{reply, START};

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
