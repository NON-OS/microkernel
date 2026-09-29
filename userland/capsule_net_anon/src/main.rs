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

//! The net.anon capsule: an Anyone Protocol onion routing transport.
//!
//! Three hops over ntor, cells across a TLS link whose peer is proved by its
//! CERTS chain, relays drawn from a consensus verified against the seven
//! authorities.

#![no_std]
#![no_main]

extern crate alloc;

mod base64_encode;
mod cell;
mod circuit;
mod crypto;
mod directory;
mod early;
mod link;
mod manager;
mod ntor;
mod path;
mod protocol;
mod server;
mod setup;
mod stream;
mod tcp_client;
mod trace;

use nonos_libc::{heap_init, mk_exit};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    trace::say(b"starting");
    early::wait_for_setup();
    trace::say(b"net.tcp found, serving");
    server::run(setup::tcp_port())
}
