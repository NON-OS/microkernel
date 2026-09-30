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

/*
 * A TCP stream through `net.anon`, over a three-hop circuit on the Anyone
 * onion network to an exit that resolves and reaches the host.
 */

use alloc::vec::Vec;

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use super::anon_call::{call, CLOSE, NOT_YET, OK, OPEN};

/* How long a stream may take to open, a circuit being built first. */
const OPEN_MS: i64 = 120_000;

pub struct AnonLink {
    pub(super) port: u32,
    pub(super) id: [u8; 2],
    pub(super) pending: Vec<u8>,
    pub(super) closed: bool,
}

impl AnonLink {
    pub fn connect(port: u32, host: &str, dst: u16) -> Result<AnonLink, ()> {
        let until = mk_uptime_ms().saturating_add(OPEN_MS);
        let body = [&dst.to_le_bytes()[..], host.as_bytes()].concat();
        loop {
            match call(port, OPEN, &body)? {
                (OK, id) if id.len() == 2 => {
                    let id = [id[0], id[1]];
                    return Ok(AnonLink { port, id, pending: Vec::new(), closed: false });
                }
                (s, _) if NOT_YET.contains(&s) && mk_uptime_ms() < until => mk_idle_ms(500),
                _ => return Err(()),
            };
        }
    }
}

impl Drop for AnonLink {
    fn drop(&mut self) {
        let _ = call(self.port, CLOSE, &self.id);
    }
}
