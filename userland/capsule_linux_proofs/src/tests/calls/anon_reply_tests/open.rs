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

//! An open without transport is unreachable, and refused otherwise.

use crate::linux::abi::errno::{ECONNREFUSED, EIO, ENETUNREACH, ENOBUFS};
use crate::linux::net::anon_answer::opened;
use crate::linux::net::anon_ops::*;

#[test]
fn an_open_is_unreachable_without_transport_and_refused_otherwise() {
    assert_eq!(opened(Some((E_OK, &[0x34, 0x12]))), Ok(0x1234));
    for status in [E_NO_TCP, E_NO_DIRECTORY, E_DIRECTORY_STALE, E_NO_PATH, E_NO_LINK, E_NO_CIRCUIT]
    {
        assert_eq!(opened(Some((status, &[]))), Err(ENETUNREACH), "status {status}");
    }
    assert_eq!(opened(Some((E_TABLE_FULL, &[]))), Err(ENOBUFS), "this capsule's share is used");
    assert_eq!(opened(Some((E_BAD_LEN, &[]))), Err(ECONNREFUSED));
    assert_eq!(opened(None), Err(EIO), "no answer");
    /* An id that is short or long, and a status nobody sends. */
    assert_eq!(opened(Some((E_OK, &[]))), Err(EIO));
    assert_eq!(opened(Some((E_OK, &[1]))), Err(EIO));
    assert_eq!(opened(Some((E_OK, &[1, 2, 3]))), Err(EIO));
    for status in [2, 3, 12, 13, 16, 17, 19, 21, u16::MAX] {
        assert_eq!(opened(Some((status, &[1, 0]))), Err(EIO), "status {status}");
    }
}
