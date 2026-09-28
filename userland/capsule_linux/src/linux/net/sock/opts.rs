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

//! The options a socket keeps, with the values Linux starts a socket with
//! (read from a Linux 6.x host: tcp_rmem[1], tcp_wmem[1], rmem_default and
//! wmem_default, and the TCP keepalive defaults).

use super::types::Proto;

#[derive(Clone, Copy)]
pub struct Opts {
    pub reuseaddr: bool,
    pub reuseport: bool,
    pub keepalive: bool,
    pub broadcast: bool,
    pub nodelay: bool,
    /// As getsockopt reports them: Linux doubles what setsockopt was given.
    pub rcvbuf: u32,
    pub sndbuf: u32,
    pub keepidle: u32,
    pub keepintvl: u32,
    pub keepcnt: u32,
    /// SO_LINGER's l_onoff and l_linger.
    pub linger: (u32, u32),
    /// SO_RCVTIMEO and SO_SNDTIMEO as the guest wrote them, seconds and
    /// microseconds, so they read back exactly.
    pub rcvtimeo: (u64, u64),
    pub sndtimeo: (u64, u64),
}

impl Opts {
    pub fn new(proto: Proto) -> Opts {
        let (rcvbuf, sndbuf) = match proto {
            Proto::Stream => (131_072, 16_384),
            Proto::Dgram => (212_992, 212_992),
        };
        Opts {
            reuseaddr: false,
            reuseport: false,
            keepalive: false,
            broadcast: false,
            nodelay: false,
            rcvbuf,
            sndbuf,
            keepidle: 7200,
            keepintvl: 75,
            keepcnt: 9,
            linger: (0, 0),
            rcvtimeo: (0, 0),
            sndtimeo: (0, 0),
        }
    }
}
