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

//! The request and reply halves of one directory fetch.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::fetch::get;
use crate::tcp_client::{send_all, wait_established};
use crate::trace;

use super::read_body::read_body;

pub(super) fn exchange(
    tcp_port: u32,
    handle: u32,
    path: &[u8],
    address: [u8; 4],
    dir_port: u16,
) -> Option<Vec<u8>> {
    if wait_established(tcp_port, handle).is_err() {
        trace::say_addr(b"dir never established", address, dir_port);
        return None;
    }
    if send_all(tcp_port, handle, &get(path)).is_err() {
        trace::say_addr(b"dir request not sent", address, dir_port);
        return None;
    }
    read_body(tcp_port, handle)
}
