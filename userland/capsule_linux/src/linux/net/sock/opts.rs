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
//! (read from a Linux 6.x host: tcp_rmem[1] and rmem_default).

use super::types::Proto;

#[derive(Clone, Copy)]
pub struct Opts {
    pub reuseaddr: bool,
    /// As getsockopt reports it; also what the socket's queue holds.
    pub rcvbuf: u32,
}

impl Opts {
    pub fn new(proto: Proto) -> Opts {
        let rcvbuf = match proto {
            Proto::Stream => 131_072,
            Proto::Dgram => 212_992,
        };
        Opts { reuseaddr: false, rcvbuf }
    }
}
