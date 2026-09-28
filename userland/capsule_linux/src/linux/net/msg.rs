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

//! `sendmsg` and `recvmsg` on a family socket: an iovec, an address, and
//! control data, which only a Unix socket carries on Linux and none here.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::fd::sock_of;
use super::flags::MSG_TRUNC;
use super::msg_hdr::{hdr, CONTROLLEN_AT, FLAGS_AT, NAMELEN_AT};
use super::policy::refuse;

/// `skip` bytes of the message went in an earlier try of this same call.
pub fn sendmsg(guest: &mut Guest, fd: u64, msg: u64, flags: u64, skip: usize) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let h = match hdr(guest, msg) {
        Ok(h) => h,
        Err(e) => return e,
    };
    if h.controllen != 0 {
        return refuse(
            "sendmsg control data on a family socket: nothing here reads it",
            errno::EINVAL,
        );
    }
    let to = match super::peer_addr::address(guest, h.name, h.namelen) {
        Ok(to) => to,
        Err(e) => return e,
    };
    if let Some(super::peer_addr::To::Inet(a)) = &to {
        if !super::sockaddr::is_loopback(a.ip) {
            return super::policy::refuse_out("sendmsg", *a);
        }
    }
    super::xfer_out::send(guest, id, &h.iov, skip, flags, to)
}

pub fn recvmsg(guest: &mut Guest, fd: u64, msg: u64, flags: u64, skip: usize) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let h = match hdr(guest, msg) {
        Ok(h) => h,
        Err(e) => return e,
    };
    let got = match super::xfer_in::recv(guest, id, &h.iov, skip, flags) {
        Ok(got) => got,
        Err(e) => return e,
    };
    let cut = if got.whole > got.n { MSG_TRUNC as u32 } else { 0 };
    let (name, lenp) = if h.name != 0 { (h.name, msg + NAMELEN_AT) } else { (0, 0) };
    let value = super::peer_addr::finish(guest, got, flags, name, lenp);
    if errno::slot(value).is_none() {
        return value;
    }
    if guest.write(msg + CONTROLLEN_AT, &0u64.to_le_bytes()) < 8
        || guest.write(msg + FLAGS_AT, &cut.to_le_bytes()) < 4
    {
        return errno::fail(errno::EFAULT);
    }
    value
}
