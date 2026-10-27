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

//! The SOCKS greeting from any caller: the install mark is read only among
//! the methods the count names, and is never the method chosen.

use crate::wire::{greeting, method_reply, METHOD_INSTALL};

#[test]
fn the_install_mark_is_read_beside_no_authentication() {
    assert_eq!(greeting(&[5, 1, 0]), Some(Ok((true, false, 3))));
    assert_eq!(greeting(&[5, 2, 0, METHOD_INSTALL]), Some(Ok((true, true, 4))));
    assert_eq!(greeting(&[5, 2, METHOD_INSTALL, 0]), Some(Ok((true, true, 4))));
    assert_eq!(greeting(&[5, 1, METHOD_INSTALL]), Some(Ok((false, true, 3))));
    assert_eq!(method_reply(true), [5, 0], "no-authentication, never the mark");
}

#[test]
fn a_mark_past_the_counted_methods_is_not_read() {
    /* One method counted; the byte after it is the next message's. */
    assert_eq!(greeting(&[5, 1, 0, METHOD_INSTALL]), Some(Ok((true, false, 3))));
    /* Counted but not arrived: wait, whatever follows. */
    assert_eq!(greeting(&[5, 3, 0, METHOD_INSTALL]), None);
    assert_eq!(greeting(&[4, 1, 0]), Some(Err(())));
}

#[test]
fn no_greeting_reads_past_what_arrived() {
    let mut buf = vec![5u8, 255];
    buf.extend_from_slice(&[METHOD_INSTALL; 255]);
    for cut in 0..buf.len() {
        assert_eq!(greeting(&buf[..cut]), None, "{cut}");
    }
    assert_eq!(greeting(&buf), Some(Ok((false, true, 257))));
}
