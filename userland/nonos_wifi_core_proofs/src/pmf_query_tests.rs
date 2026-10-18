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

//! The SA Query on a protected link: the station's question goes out under
//! the pairwise key and a protected answer echoing it keeps the link; silence
//! ends it; the AP's own question is answered.

use crate::ap_sim::{AA, SPA};
use crate::pmf_frames::{mgmt, sealed};
use crate::pmf_sim::{joined_pmf, tk};
use nonos_wifi_core::dot11::ccmp::decrypt;
use nonos_wifi_core::station::{MgmtDrop, MgmtRx, SaTick};

const ACTION: u8 = 13;

#[test]
fn a_protected_answer_to_the_stations_query_keeps_the_link() {
    let (mut s, ptk) = joined_pmf();
    let req = s.sa_query_start(0, [1, 2]).expect("a request");
    assert_eq!((&req[4..10], &req[10..16]), (&AA[..], &SPA[..]));
    assert_eq!(decrypt(&req, &tk(&ptk)).unwrap(), vec![8, 0, 1, 2]);
    assert!(s.sa_query_start(1, [3, 4]).is_none(), "one at a time");
    let stranger = sealed(&mgmt(ACTION, SPA, &[8, 1, 9, 9]), 1, &ptk, false);
    assert_eq!(s.receive_mgmt(&stranger, false), MgmtRx::Dropped(MgmtDrop::NotHandled));
    let clear = mgmt(ACTION, SPA, &[8, 1, 1, 2]);
    assert_eq!(s.receive_mgmt(&clear, false), MgmtRx::Dropped(MgmtDrop::Unprotected));
    let answer = sealed(&mgmt(ACTION, SPA, &[8, 1, 1, 2]), 2, &ptk, false);
    assert_eq!(s.receive_mgmt(&answer, false), MgmtRx::Confirmed);
    assert_eq!(s.sa_query_poll(5000, [5, 5]), SaTick::Idle);
    assert!(s.is_associated());
}

#[test]
fn silence_retries_then_ends_the_link() {
    let (mut s, ptk) = joined_pmf();
    s.sa_query_start(0, [1, 1]).unwrap();
    assert_eq!(s.sa_query_poll(100, [2, 2]), SaTick::Idle);
    let SaTick::Send(retry) = s.sa_query_poll(206, [2, 2]) else { panic!("a retry") };
    assert_eq!(decrypt(&retry, &tk(&ptk)).unwrap(), vec![8, 0, 2, 2]);
    assert_eq!(s.sa_query_poll(1024, [3, 3]), SaTick::Ended);
    assert!(!s.is_associated());
    assert_eq!(s.sa_query_poll(2000, [4, 4]), SaTick::Idle);
}

#[test]
fn the_aps_question_is_answered_once() {
    let (mut s, ptk) = joined_pmf();
    let q = sealed(&mgmt(ACTION, SPA, &[8, 0, 5, 6]), 1, &ptk, true);
    let MgmtRx::Reply(r) = s.receive_mgmt(&q, true) else { panic!("an answer") };
    assert_eq!(decrypt(&r, &tk(&ptk)).unwrap(), vec![8, 1, 5, 6]);
    assert_eq!(s.receive_mgmt(&q, true), MgmtRx::Dropped(MgmtDrop::Replay));
}
