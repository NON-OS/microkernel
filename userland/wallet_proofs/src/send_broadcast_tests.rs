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

//! What a broadcast came to: a taken one followed under its own hash, one
//! whose answer was lost said as unknown and followed, never sent again.

use crate::wallet::send::outcome::{judge, Broadcast, Carried, REFUSED};

const LOCAL: [u8; 32] = [0x11; 32];
const OTHER: [u8; 32] = [0x22; 32];

#[test]
fn a_taken_broadcast_is_followed_under_its_own_hash() {
    let b = judge(Carried::Answer(Some(LOCAL)), &LOCAL);
    assert_eq!(b, Broadcast::Sent(LOCAL));
    assert_eq!(b.follow(), Some(LOCAL));
    /* A node that names another hash cannot point the wallet elsewhere. */
    assert_eq!(judge(Carried::Answer(Some(OTHER)), &LOCAL).follow(), Some(LOCAL));
}

#[test]
fn a_broadcast_whose_answer_was_lost_is_unknown_and_followed_not_resent() {
    let b = judge(Carried::Unknown, &LOCAL);
    assert_eq!(b, Broadcast::Unknown(LOCAL));
    assert_eq!(b.follow(), Some(LOCAL), "its receipt is followed as a sent one's is");
    assert!(!matches!(b, Broadcast::NotSent(_)), "never read as one that did not go");
}

#[test]
fn a_refusal_and_a_route_that_carried_nothing_follow_nothing() {
    assert_eq!(judge(Carried::Answer(None), &LOCAL), Broadcast::Refused(REFUSED));
    assert_eq!(judge(Carried::Answer(None), &LOCAL).follow(), None);
    let funds = "not enough";
    assert_eq!(judge(Carried::Refused(funds), &LOCAL), Broadcast::Refused(funds));
    assert_eq!(judge(Carried::Refused(funds), &LOCAL).follow(), None);
    let why = "the Nym mixnet is not connected yet";
    assert_eq!(judge(Carried::NotSent(why), &LOCAL), Broadcast::NotSent(why));
    assert_eq!(judge(Carried::NotSent(why), &LOCAL).follow(), None);
}
