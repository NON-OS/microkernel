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

//! A socket as it starts: unbound, unconnected, Linux's default options.

use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;

use super::opts::Opts;
use super::types::{Domain, Proto, Sock};

impl Sock {
    pub fn new(domain: Domain, proto: Proto, pid: Option<u32>) -> Sock {
        Sock {
            domain,
            proto,
            local: None,
            remote: None,
            listening: false,
            backlog: 0,
            pending: VecDeque::new(),
            peer: None,
            connected: false,
            rx: VecDeque::new(),
            grams: VecDeque::new(),
            eof: false,
            wr_shut: false,
            rd_shut: false,
            broken: false,
            error: 0,
            opts: Opts::new(proto),
            svc: None,
            holders: pid.map_or_else(Vec::new, |p| vec![p]),
            uname: None,
            upeer: None,
        }
    }
}
