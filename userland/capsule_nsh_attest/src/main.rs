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

//! nsh.attest. It holds AttestRead, Crypto and FileSystem, and not
//! Network, and answers nsh.front alone: the kernel's attestation document
//! for each handshake's challenge and the registry's entries; the host's
//! hybrid public keys and its signatures, made with keys this machine made
//! at its first boot and sealed to its TPM; and the machine's own records
//! for each connection. No host secret leaves this capsule.

#![no_std]
#![no_main]

extern crate alloc;

mod anchor;
mod attest;
mod caller;
mod challenge;
mod keys;
mod records;
mod reply;
mod request;
mod rng;
mod said;
mod server;
mod store;

use nonos_libc::{heap_init, mk_debug, mk_exit};

/// Write one line about the host keys to the serial console. Lines need
/// Debug, which only a development twin grants; elsewhere this is a no-op.
pub(crate) fn say(e: said::Event, key_id: Option<&[u8; 32]>) {
    let line = said::line(e, key_id);
    let _ = mk_debug(line.as_ptr(), line.len());
}

/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    server::run();
}
