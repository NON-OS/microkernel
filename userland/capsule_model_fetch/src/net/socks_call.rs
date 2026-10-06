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
 * One exchange with `net.socks5`, which speaks SOCKS5 as bytes over IPC.
 * Every request and every answer carries one marker byte first, so that an
 * empty ask (has anything come?) and an empty answer (nothing yet) can be
 * sent at all; the answer's marker also says whether the far end closed.
 */

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

pub const BYTES: u8 = 0;
pub const RESET: u8 = 1;
pub const CLOSED: u8 = 1;
/* A mixnet round trip crosses several hops before an exit answers. */
const SEND_MS: u64 = 15_000;
const ASK_MS: u64 = 2_000;
const ANSWER_MAX: usize = 36 * 1024;

/* Send `framed` (marker first) and take the answer, marker first. */
pub fn call(port: u32, framed: &[u8]) -> Result<Vec<u8>, ()> {
    let wait = if framed.len() <= 1 { ASK_MS } else { SEND_MS };
    let mut rx = vec![0u8; ANSWER_MAX];
    let n = mk_ipc_call_timeout(
        port as u64,
        framed.as_ptr(),
        framed.len(),
        rx.as_mut_ptr(),
        rx.len(),
        wait,
    );
    if n < 1 {
        return Err(());
    }
    rx.truncate(n as usize);
    Ok(rx)
}
