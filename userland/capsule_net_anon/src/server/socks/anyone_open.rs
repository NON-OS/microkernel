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

//! Opening an Anyone stream for a SOCKS CONNECT.

use crate::manager::{open_stream, Manager, SendError};
use crate::trace;

use super::super::handlers::not_ready;
use super::rep::{REP_FAILURE, REP_NET_UNREACHABLE};

/// The stream id, or the SOCKS reply code that says why there is none.
///
/// The caller is told the network is not there yet, and the log says which
/// part of it is missing: the directory, a path, or a link.
pub fn open(state: &mut Manager, now: u64, host: &[u8], port: u16) -> Result<u16, u8> {
    if let Some(why) = not_ready(state, now) {
        trace::say_num(b"socks connect refused, not ready, errno", why as u64);
        return Err(REP_NET_UNREACHABLE);
    }
    match open_stream(state, host, port, now) {
        Ok(id) => Ok(id),
        Err(SendError::NoLink | SendError::NoCircuit) => {
            trace::say(b"socks connect refused, no circuit");
            Err(REP_NET_UNREACHABLE)
        }
        Err(_) => {
            trace::say(b"socks connect refused, stream table full");
            Err(REP_FAILURE)
        }
    }
}
