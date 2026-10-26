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

//! Reaching the network without Network, and the lookup every attempt on a
//! named service starts from.

use nonos_libc::mk_syscall_raw as raw;

use crate::codes::{MISD, MMMP, MSVL};
use crate::line::{note, verdict};

/// Without Network, open a socket through net.sockets; without Mmio or Admin,
/// ask for a device's MMIO window.
pub fn cap_escape() {
    // Every service that carries traffic takes Network
    // (src/services/registry/policy.rs), the anonymity ones included: a
    // capsule without it must not reach the network through any of them.
    for (name, what) in NETWORK_SERVICES {
        network_send(name, what);
    }
    let mut map = [0u8; 32];
    let rc = raw(MMMP, [0, 0, 0, 0, 4096, map.as_mut_ptr() as u64]);
    verdict(b"cap-escape", b"an MMIO window without Mmio, MkMmioMap", rc);
}

const NETWORK_SERVICES: [(&[u8], &[u8]); 6] = [
    (b"net.sockets", b"a socket through net.sockets without Network, MkIpcSend"),
    (b"net.tcp", b"a TCP connection through net.tcp without Network, MkIpcSend"),
    (b"net.dns", b"a name lookup through net.dns without Network, MkIpcSend"),
    (b"net.nym", b"a mixnet session through net.nym without Network, MkIpcSend"),
    (b"net.anon", b"an onion circuit through net.anon without Network, MkIpcSend"),
    (b"net.socks5", b"a SOCKS stream through net.socks5 without Network, MkIpcSend"),
];

fn network_send(name: &[u8], what: &[u8]) {
    if let Some((port, _)) = lookup(name) {
        let open = [0u8; 16];
        let rc = raw(MISD, [u64::from(port), open.as_ptr() as u64, open.len() as u64, 0, 0, 0]);
        verdict(b"cap-escape", what, rc);
    } else {
        note(
            b"[ATTACK-NOTE] cap-escape: a network service is not running, its send was not tried\n",
        );
    }
}

/// A service's port and pid, when it is running.
pub(crate) fn lookup(name: &[u8]) -> Option<(u32, u32)> {
    let (mut port, mut pid) = (0u32, 0u32);
    let found = raw(
        MSVL,
        [
            name.as_ptr() as u64,
            name.len() as u64,
            &mut port as *mut u32 as u64,
            &mut pid as *mut u32 as u64,
            0,
            0,
        ],
    );
    (found == 0 && port != 0 && pid != 0).then_some((port, pid))
}
