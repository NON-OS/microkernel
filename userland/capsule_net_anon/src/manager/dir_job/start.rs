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

//! Opening the connection a directory fetch runs on.

extern crate alloc;

use nonos_libc::mk_uptime_ms;

use crate::directory::fetch::get;
use crate::tcp_client::connect;
use crate::tcp_client::errno::{E_ERRNO, NOT_UP, NO_ROUTE};
use crate::trace;

use super::job::{Job, Stage, ESTABLISH_MS};

/// Ask net.tcp for a connection to an authority. `None`, traced, when it
/// refuses at once. Nothing here waits: the connection opens over the turns
/// that follow.
pub(super) fn start(tcp_port: u32, address: [u8; 4], dir_port: u16, path: &[u8]) -> Option<Job> {
    match connect(tcp_port, address, dir_port) {
        Ok(handle) => Some(Job {
            handle,
            address,
            dir_port,
            request: get(path),
            sent: 0,
            stage: Stage::Opening,
            deadline: mk_uptime_ms().saturating_add(ESTABLISH_MS),
            opening: false,
            raw: alloc::vec::Vec::new(),
        }),
        /*
         * Not "refused": nothing at the far end ever saw these. A network
         * that is not up yet is said once per sweep by the sweep, not here
         * once per authority.
         */
        Err(errno) => {
            if errno == E_ERRNO + NOT_UP || errno == E_ERRNO + NO_ROUTE {
                super::super::cold_wait::not_up();
            } else {
                trace::say_addr(b"dir connect failed", address, dir_port);
                trace::say_num(b"dir connect errno", errno as u64);
            }
            None
        }
    }
}
