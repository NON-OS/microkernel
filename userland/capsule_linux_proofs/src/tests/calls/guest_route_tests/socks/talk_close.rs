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

//! A conversation ended without a stream: told why in its own version, or
//! not at all when it is not SOCKS, and refused past any request's length.

use super::{name, v5_request};
use crate::linux::net::routed::socks::kinds::REQUEST_MAX;
use crate::linux::net::routed::socks::{Heard, Talk, Ver};

#[test]
fn a_greeting_without_no_authentication_is_told_so_and_closed() {
    let (mut t, mut said) = (Talk::default(), Vec::new());
    assert_eq!(t.hear(&[5, 1, 2], &mut said), (3, Heard::Closed));
    assert_eq!(said, [5, 0xFF]);
}

#[test]
fn refusals_are_said_in_the_request_s_own_version_then_closed() {
    for (bytes, want) in [
        ([&[5u8, 1, 0][..], &[5, 2, 0, 1, 1, 2, 3, 4, 0, 80]].concat(), vec![5, 0, 5, 7]),
        ([&[5u8, 1, 0][..], &[5, 1, 0, 4]].concat(), vec![5, 0, 5, 8]),
        (vec![4, 2, 0, 80, 1, 2, 3, 4, 0], vec![0, 91]),
    ] {
        let (mut t, mut said) = (Talk::default(), Vec::new());
        assert_eq!(t.hear(&bytes, &mut said).1, Heard::Closed);
        assert_eq!(said[..want.len()], want[..], "{bytes:?}");
    }
}

#[test]
fn what_is_not_socks_is_closed_without_a_word() {
    for first in (0u8..=255).filter(|&b| b != 4 && b != 5) {
        let (mut t, mut said) = (Talk::default(), Vec::new());
        assert_eq!(t.hear(&[first, 1, 0], &mut said), (3, Heard::Closed));
        assert!(said.is_empty(), "first byte {first}");
    }
}

#[test]
fn a_request_longer_than_any_is_refused_and_never_held_past_its_bound() {
    /* SOCKS4's user id has no bound of its own. */
    let flood = [&[4u8, 1, 0, 80, 1, 2, 3, 4][..], &[b'x'; 4096]].concat();
    let (mut t, mut said) = (Talk::default(), Vec::new());
    assert_eq!(t.hear(&flood, &mut said), (REQUEST_MAX, Heard::Closed));
    assert_eq!(said, [0, 91, 0, 0, 0, 0, 0, 0]);
    assert_eq!(t.hear(b"more", &mut said), (0, Heard::Closed), "nothing past the bound is held");
}

#[test]
fn the_longest_socks5_greeting_and_request_still_pass_in_one_write() {
    let long = "a".repeat(255);
    let greeting = [&[5u8, 255][..], &[0u8; 255]].concat();
    let both = [&greeting[..], &v5_request(1, &name(&long, 9050).to, 9050)].concat();
    assert!(both.len() > REQUEST_MAX);
    let (mut t, mut said) = (Talk::default(), Vec::new());
    assert_eq!(t.hear(&both, &mut said), (REQUEST_MAX, Heard::More));
    let (took, heard) = t.hear(&both[REQUEST_MAX..], &mut said);
    assert_eq!(took, both.len() - REQUEST_MAX);
    assert_eq!(heard, Heard::Connect(Ver::Five, name(&long, 9050)));
    assert_eq!(said, [5, 0]);
}
