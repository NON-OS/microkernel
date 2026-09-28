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

//! Options Linux keeps that change nothing a loopback connection can see:
//! the IP type of service and time to live, the priority a queueing
//! discipline would use, and TCP's user timeout, quick-ack and Fast Open
//! queue. Each is kept, checked as Linux checks it, and read back.

use crate::linux::abi::errno;

use super::ids::{
    IPPROTO_IP, IPPROTO_TCP, IP_TOS, IP_TTL, SOL_SOCKET, SO_PRIORITY, TCP_FASTOPEN, TCP_QUICKACK,
    TCP_USER_TIMEOUT,
};
use crate::linux::net::sock::{More, Proto};

/// Linux's net.ipv4.ip_default_ttl.
const DEFAULT_TTL: u32 = 64;
/// The ECN bits of the type of service, which a TCP socket does not keep.
const ECN_MASK: u32 = 3;

pub fn known(level: u64, name: u64) -> bool {
    matches!(
        (level, name),
        (IPPROTO_IP, IP_TOS | IP_TTL)
            | (SOL_SOCKET, SO_PRIORITY)
            | (IPPROTO_TCP, TCP_USER_TIMEOUT | TCP_QUICKACK | TCP_FASTOPEN)
    )
}

pub fn set(m: &mut More, proto: Proto, level: u64, name: u64, v: u32) -> u64 {
    match (level, name) {
        (IPPROTO_IP, IP_TOS) if proto == Proto::Stream => m.tos = v & 0xff & !ECN_MASK,
        (IPPROTO_IP, IP_TOS) => m.tos = v & 0xff,
        (IPPROTO_IP, IP_TTL) if v as i32 == -1 => m.ttl = DEFAULT_TTL,
        (IPPROTO_IP, IP_TTL) if (1..=255).contains(&v) => m.ttl = v,
        (SOL_SOCKET, SO_PRIORITY) => m.priority = v,
        (IPPROTO_TCP, TCP_USER_TIMEOUT) if (v as i32) >= 0 => m.user_timeout = v,
        (IPPROTO_TCP, TCP_QUICKACK) => m.quickack = v != 0,
        (IPPROTO_TCP, TCP_FASTOPEN) if (v as i32) >= 0 => m.fastopen = v,
        _ => return errno::fail(errno::EINVAL),
    }
    errno::ok(0)
}

pub fn get(m: &More, level: u64, name: u64) -> u32 {
    match (level, name) {
        (IPPROTO_IP, IP_TOS) => m.tos,
        (IPPROTO_IP, IP_TTL) => m.ttl,
        (SOL_SOCKET, SO_PRIORITY) => m.priority,
        (IPPROTO_TCP, TCP_USER_TIMEOUT) => m.user_timeout,
        (IPPROTO_TCP, TCP_QUICKACK) => u32::from(m.quickack),
        _ => m.fastopen,
    }
}
