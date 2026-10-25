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

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use crate::term::state::State;

const SERVICE: &[u8] = b"net.dns";
const HDR_LEN: usize = 20;
const TIMEOUT_MS: u64 = 2000;

#[path = "nslookup_ask.rs"]
mod nslookup_ask;
#[path = "nslookup_gate.rs"]
mod nslookup_gate;
#[path = "nslookup_said.rs"]
mod nslookup_said;

use nslookup_ask::request;
use nslookup_gate::may_ask;
use nslookup_said::answer_line;

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    let Some(&host) = args.first() else {
        state.scrollback.push_error(b"usage: nslookup <host>");
        return false;
    };
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(
        SERVICE.as_ptr(),
        SERVICE.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    if rc < 0 || port == 0 {
        state.scrollback.push_error(b"nslookup: dns unavailable");
        return false;
    }
    if !may_ask(state) {
        return false;
    }
    let tx = request(host);
    let mut rx = [0u8; HDR_LEN + 4];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < (HDR_LEN + 4) as i64 || u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        state.scrollback.push_error(b"nslookup: not resolved");
        return false;
    }
    let line = answer_line(host, &rx);
    state.scrollback.push_line(&line);
    true
}
