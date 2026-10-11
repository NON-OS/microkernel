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

//! Round the held gates by a reply endpoint. Every name a
//! capsule owns is read from its one inbox, so a send to a driver's reply
//! port is a send to the driver: before K7 the kernel checked only the
//! name used, which needs IPC, and xhci even answered bulk-IN.

use nonos_libc::mk_syscall_raw as raw;

use crate::codes::{MICL, MISD};
use crate::line::verdict;

/// The reply ports of the 15 held drivers (src/hardware/*_capsule and
/// src/userspace/capsule_driver_*/spawn.rs, REPLY_PORT).
const HELD_REPLIES: [u16; 15] =
    [4201, 4205, 4207, 4209, 4211, 4213, 4215, 4219, 4223, 4225, 4227, 4229, 4231, 4233, 4235];
/// The net.* services' reply ports, each held to Network.
const NET_REPLIES: [u16; 11] = [4401, 4403, 4421, 4431, 4441, 4451, 4461, 4471, 4481, 4483, 4485];

/// A request frame: magic, version 1, op, flags 0, reserved 0, id 1, length.
fn frame(magic: u32, op: u16, body: &[u8]) -> [u8; 24] {
    let mut f = [0u8; 24];
    f[..4].copy_from_slice(&magic.to_le_bytes());
    f[4..6].copy_from_slice(&1u16.to_le_bytes());
    f[6..8].copy_from_slice(&op.to_le_bytes());
    f[12..16].copy_from_slice(&1u32.to_le_bytes());
    f[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    f[20..20 + body.len()].copy_from_slice(body);
    f
}

fn call(port: u16, f: &[u8], what: &[u8]) {
    let mut back = [0u8; 512];
    let args = [u64::from(port), f.as_ptr() as u64, f.len() as u64, back.as_mut_ptr() as u64];
    let rc = raw(MICL, [args[0], args[1], args[2], args[3], back.len() as u64, 500]);
    verdict(b"held-escape", what, rc);
}

pub fn reply_escape() {
    let junk = [0u8; 16];
    for port in HELD_REPLIES {
        let rc = raw(MISD, [u64::from(port), junk.as_ptr() as u64, junk.len() as u64, 0, 0, 0]);
        verdict(b"held-escape", b"a held driver by its reply port, MkIpcSend", rc);
    }
    for port in NET_REPLIES {
        let rc = raw(MISD, [u64::from(port), junk.as_ptr() as u64, junk.len() as u64, 0, 0, 0]);
        verdict(b"held-escape", b"a net service by its reply port without Network, MkIpcSend", rc);
    }
    let xhci = 0x4E58_4843;
    call(4207, &frame(xhci, 0x0003, &[]), b"xhci's port status by its reply port, MkIpcCall");
    call(
        4207,
        &frame(xhci, 0x0012, &[1, 0, 13, 0]),
        b"xhci's bulk-IN by its reply port, MkIpcCall",
    );
    let msc = frame(0x4E55_4D53, 0x0008, &[]);
    call(4225, &msc, b"usb_msc's state by its reply port, MkIpcCall");
}
