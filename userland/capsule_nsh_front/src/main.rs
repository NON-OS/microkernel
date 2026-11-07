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

//! nsh.front. It holds Network (and not AttestRead) and serves nsh/1: a
//! net.sockets stream listener, the sans-IO `nsh_server` driven per
//! connection, the attestation challenge, the host's signatures and the
//! machine's records relayed through nsh.attest, and a minimal shell behind
//! the sealed session. It holds no host secret. Setup resolves its two peers
//! (net.sockets, nsh.attest) and the listener before the first client.

#![no_std]
#![no_main]

extern crate alloc;

mod accepted;
mod attest;
mod clock;
mod host;
mod limits;
mod line;
mod marks;
mod net_ask;
mod net_rule;
mod newcomers;
mod procs;
mod read;
mod records;
mod refused_said;
mod relay;
mod relay_said;
mod reply;
mod rng;
mod server;
mod session;
mod setup;
mod shell;
mod shell_host;
mod sock;
mod turn_away;
mod unready;

use nonos_libc::{heap_init, mk_exit};

/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    server::run();
}
