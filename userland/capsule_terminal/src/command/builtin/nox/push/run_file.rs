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

//! One file pushed: read from the store and sent as one PUT.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::read_file;

use super::super::args::PushArgs;
use super::super::request;
use crate::term::state::State;

const MAX: u32 = 64 * 1024 * 1024;

/// The file at `path` sent to `a.target`, said either way.
pub(super) fn push_file(
    state: &mut State,
    pid: u32,
    ip: [u8; 4],
    path: Vec<u8>,
    a: &PushArgs,
    extra: Vec<u8>,
) -> bool {
    /* One byte past the most, so a longer file is refused, never sent cut. */
    let body = match read_file(pid, &path, MAX + 1) {
        Ok(b) if b.len() > MAX as usize => {
            state.scrollback.push_error(b"larger than a push sends (64 MiB)");
            return false;
        }
        Ok(b) => b,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    match request::put(ip, a.target.port, &a.target.host, &a.target.path, &body, &extra) {
        Ok(()) => {
            let mut msg = Vec::from(&b"pushed "[..]);
            msg.extend_from_slice(&a.target.path);
            state.scrollback.push_line(&msg);
            true
        }
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            false
        }
    }
}
