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

//! `recvfrom`: a read that says where the bytes came from.

use alloc::vec;

use crate::linux::guest::Guest;

use super::dgram::is_resolver;
use super::fd::sock_of;

pub fn recvfrom(
    guest: &mut Guest,
    fd: u64,
    buf: u64,
    len: u64,
    flags: u64,
    at: u64,
    alen: u64,
) -> u64 {
    if is_resolver(guest, fd) {
        return super::resolver::answer(guest, fd, buf, len, at, alen);
    }
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    match super::xfer_in::recv(guest, id, &vec![(buf, len)], 0, flags) {
        Ok(got) => super::peer_addr::finish(guest, got, flags, at, alen),
        Err(e) => e,
    }
}
