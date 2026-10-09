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

//! One HTTP GET to a DirPort over net.tcp.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::fetch::body;
use crate::tcp_client::errno::{E_ERRNO, LOCAL_PORT_IN_USE, UNADDRESSABLE};
use crate::tcp_client::{close, connect};
use crate::trace;

use super::http_exchange::exchange;

/*
 * Every failure here retries against the next authority, so the return stayed
 * a bare None for a long time on the grounds that the action is the same. The
 * action is; the diagnosis is not.
 */

/// Fetch and inflate `path` from an authority. `None` on any failure, traced.
pub fn fetch(tcp_port: u32, address: [u8; 4], dir_port: u16, path: &[u8]) -> Option<Vec<u8>> {
    let handle = match connect(tcp_port, address, dir_port) {
        Ok(handle) => handle,
        Err(errno) => {
            // The errno as well as the address. Which of net.tcp's refusals this is decides
            // where to look next.
            // Not "refused": nothing at the far end ever saw these. The word sent a boot
            // looking at the network for a stack with no address yet.
            match errno {
                e if e == E_ERRNO + UNADDRESSABLE => {
                    trace::say_addr(b"dir connect has no route yet", address, dir_port)
                }
                e if e == E_ERRNO + LOCAL_PORT_IN_USE => {
                    trace::say_addr(b"dir connect local port in use", address, dir_port)
                }
                _ => {
                    trace::say_addr(b"dir connect failed", address, dir_port);
                    trace::say_num(b"dir connect errno", errno as u64);
                }
            }
            return None;
        }
    };
    let raw = exchange(tcp_port, handle, path, address, dir_port);
    let _ = close(tcp_port, handle);
    match body(&raw?) {
        Ok(out) if out.is_empty() => {
            trace::say_addr(b"dir answered with nothing", address, dir_port);
            None
        }
        Ok(out) => Some(out),
        Err(_) => {
            trace::say_addr(b"dir body not usable", address, dir_port);
            None
        }
    }
}
