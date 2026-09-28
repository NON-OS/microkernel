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

//! A socket's address written into guest memory.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::sock::Peer;
use super::sockaddr::{AF_INET, AF_UNIX};

/// Write `peer` at `at`, cut to the length the guest offered at `lenp`, and
/// the whole length back at `lenp`, as Linux's move_addr_to_user does. An
/// unnamed Unix socket is the family alone; a path carries its NUL.
pub fn write(guest: &mut Guest, at: u64, lenp: u64, peer: &Peer) -> u64 {
    if at == 0 || lenp == 0 {
        return errno::ok(0);
    }
    let Some(raw) = guest.read(lenp, 4) else {
        return errno::fail(errno::EFAULT);
    };
    let room = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]) as i32;
    if room < 0 {
        return errno::fail(errno::EINVAL);
    }
    let sa = encode(peer);
    let n = sa.len().min(room as usize);
    if guest.write(at, &sa[..n]) < n as i64
        || guest.write(lenp, &(sa.len() as u32).to_le_bytes()) < 4
    {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}

fn encode(peer: &Peer) -> Vec<u8> {
    let mut sa = Vec::with_capacity(16);
    match peer {
        Peer::Inet(addr) => {
            sa.extend_from_slice(&AF_INET.to_le_bytes());
            sa.extend_from_slice(&addr.port.to_be_bytes());
            sa.extend_from_slice(&addr.ip);
            sa.resize(16, 0);
        }
        Peer::Unix(name) => {
            sa.extend_from_slice(&AF_UNIX.to_le_bytes());
            if let Some(n) = name {
                sa.extend_from_slice(&n.shown);
                if !n.is_abstract() {
                    sa.push(0);
                }
            }
        }
    }
    sa
}
