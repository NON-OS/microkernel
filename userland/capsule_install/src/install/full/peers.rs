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
 * The compositor and the input router, waited for rather than asked once:
 * both start before setup, so on an install boot they are there, and the
 * wait bounds only a desktop that never came.
 */

use nonos_app_skeleton::discover::lookup_port;
use nonos_libc::{mk_yield, Deadline};

const PEERS_WAIT_MS: u64 = 60_000;

/* (compositor, input router) ports. */
pub(super) fn wait() -> Result<(u32, u32), &'static str> {
    let until = Deadline::after_ms(PEERS_WAIT_MS);
    loop {
        match (lookup_port(b"compositor"), lookup_port(b"input_router")) {
            (Some(c), Some(r)) => return Ok((c, r)),
            _ if until.expired() => return Err("no compositor or input router"),
            _ => {
                let _ = mk_yield();
            }
        }
    }
}
