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

//! The capsule's wire to the host controller: a reply is read only as far
//! as it came, whatever length its header names.

use nonos_libc::set_reply;

use crate::wire::{call, E_IO, HDR_LEN, OP_PORT_STATUS};

/// A reply whose header names `payload` bytes, carrying `data` after the
/// status.
fn reply(payload: u32, status: i32, data: &[u8]) -> Vec<u8> {
    let mut r = vec![0u8; HDR_LEN];
    r[0..4].copy_from_slice(&0x4E58_4843u32.to_le_bytes());
    r[16..20].copy_from_slice(&payload.to_le_bytes());
    r.extend_from_slice(&status.to_le_bytes());
    r.extend_from_slice(data);
    r
}

#[test]
fn a_whole_reply_gives_its_status_and_data() {
    set_reply(reply(7, 0, &[1, 2, 3]));
    let mut data = [0u8; 8];
    assert_eq!(call(1, OP_PORT_STATUS, &[], &mut data), (0, 3));
    assert_eq!(data[..3], [1, 2, 3]);
}

#[test]
fn a_reply_naming_more_than_came_is_an_io_error() {
    set_reply(reply(4 + 64, 0, &[9; 8]));
    let mut data = [0u8; 64];
    assert_eq!(call(1, OP_PORT_STATUS, &[], &mut data), (E_IO, 0));
    assert!(data.iter().all(|&b| b == 0), "nothing copied");
}

#[test]
fn a_reply_too_short_for_its_status_is_an_io_error() {
    set_reply(reply(2, 0, &[]));
    assert_eq!(call(1, OP_PORT_STATUS, &[], &mut [0u8; 4]), (E_IO, 0));
}
