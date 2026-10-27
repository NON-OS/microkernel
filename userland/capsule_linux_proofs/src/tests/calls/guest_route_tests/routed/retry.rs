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

//! The retry rule: only a network that has not come up is asked again, a
//! second apart, and never past route link's own OPEN_MS from the connect.

use crate::bounds::{NOT_YET_GAP_MS, OPEN_MS};
use crate::linux::net::anon_ops::*;
use crate::linux::net::routed::retry::{again, not_ready, refused, until};

const NOT_UP: [u16; 6] =
    [E_NO_TCP, E_NO_DIRECTORY, E_DIRECTORY_STALE, E_NO_PATH, E_NO_LINK, E_NO_CIRCUIT];

#[test]
fn the_bound_is_the_one_an_install_waits() {
    assert_eq!(OPEN_MS, 45_000);
    assert_eq!(NOT_YET_GAP_MS, 1_000);
    assert_eq!(until(1_234), 1_234 + 45_000);
    assert_eq!(until(i64::MAX), i64::MAX, "no overflow");
}

#[test]
fn only_a_network_not_up_yet_is_asked_again() {
    for status in 0u16..=512 {
        let waited = not_ready(status);
        assert_eq!(waited.is_some(), NOT_UP.contains(&status), "status {status}");
        if let Some(why) = waited {
            assert!(!why.is_empty() && why.len() < 80, "{why}");
        }
    }
    /* A refusal, a full table, a garbled request: told at once, never waited out. */
    for status in [E_OK, E_BAD_LEN, E_TABLE_FULL, E_NO_STREAM, E_STREAM_CLOSED, E_WOULD_BLOCK] {
        assert_eq!(not_ready(status), None, "status {status}");
    }
    assert_ne!(refused(Some(E_TABLE_FULL)), refused(None));
}

#[test]
fn an_open_is_asked_again_a_second_apart_until_its_time_is_up_and_no_later() {
    for since in [0i64, 7_777, 1 << 40] {
        let end = until(since);
        let (mut now, mut tries) = (since, 1);
        while let Some(at) = again(end, now) {
            assert!(at > now && at <= end, "{since}: {now} -> {at}");
            assert!(at - now <= NOT_YET_GAP_MS as i64);
            (now, tries) = (at, tries + 1);
        }
        assert_eq!(now, end, "the last try is at the bound");
        assert_eq!(tries, 46, "one at the connect and one each second for 45 s");
        assert_eq!(again(end, end), None);
        assert_eq!(again(end, end + 1), None);
    }
}
