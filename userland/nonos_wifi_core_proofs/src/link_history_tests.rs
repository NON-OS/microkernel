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
//! W1 (c): the history a driver keeps of its link, as the link reply carries
//! it: lost or left, the joins finished counted past 255 without reaching 0,
//! and a join that came up clearing how the last link ended.

use nonos_wifi_core::link_history::{Ended, LinkHistory, CODE_LEFT, TAIL_LEN};

#[test]
fn the_history_is_written_byte_for_byte() {
    let mut h = LinkHistory::NEW;
    let mut out = [0xEEu8; TAIL_LEN + 1];
    assert_eq!(h.encode(&mut out), Some(TAIL_LEN));
    assert_eq!(out[..TAIL_LEN], [0, 0, 0, 0, 0, 0], "nothing yet");
    h.joined(-10);
    h.lost();
    h.encode(&mut out).unwrap();
    assert_eq!(out[..TAIL_LEN], [1, 1, 0xF6, 0xFF, 0xFF, 0xFF]);
    h.left();
    h.encode(&mut out).unwrap();
    assert_eq!(out[0], 2);
    assert_eq!(out[TAIL_LEN], 0xEE, "nothing past the tail");
    assert_eq!(h.encode(&mut out[..TAIL_LEN - 1]), None, "no room, nothing written");
}

#[test]
fn a_join_that_came_up_clears_the_end_and_the_count_skips_zero() {
    let mut h = LinkHistory::NEW;
    h.left();
    h.joined(-2);
    assert_eq!(h.ended, Ended::Left, "a failed join leaves it as it was");
    h.joined(0);
    assert_eq!((h.ended, h.code), (Ended::No, 0));
    for _ in 0..300 {
        h.joined(0);
        assert_ne!(h.joins, 0, "0 means no join at all");
    }
}

#[test]
fn only_a_link_the_person_left_refuses_a_rejoin_until_a_join_comes_up() {
    let mut h = LinkHistory::NEW;
    assert!(!h.refuses_rejoin(), "nothing yet");
    h.lost();
    assert!(!h.refuses_rejoin(), "a lost link is rejoined");
    h.left();
    assert!(h.refuses_rejoin());
    h.joined(CODE_LEFT);
    assert!(h.refuses_rejoin(), "the refusal itself does not clear it");
    h.joined(0);
    assert!(!h.refuses_rejoin(), "joined again, from Settings");
    assert_eq!(CODE_LEFT, -13);
}
