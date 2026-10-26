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

//! The desktop shell as a deputy. It is a launcher (SpawnWindow), and any
//! process may send it OP_OPEN_WITH, so it starts only an app that opens
//! files. Asked for the installer, it answers ENOENT with its own
//! `[CAP-DENY] pid=<this pid> open-with:` line, and nothing is started.

use nonos_libc::mk_ipc_call_timeout;

use crate::deputy_port::shell_port;
use crate::line::{escaped, note, verdict};

const NDSH: u32 = 0x4E44_5348;
const OP_OPEN_WITH: u16 = 0x0007;
const APP: &[u8] = b"app.install";
const PATH: &[u8] = b"/home/nonos/readme.txt";
const WHAT: &[u8] = b"app.install through the shell's OP_OPEN_WITH";
/// How long the shell may take: it serves the desktop while the boot is
/// still busy, and at 5 s (the kernel's default) it answered after the call.
const SHELL_MS: u64 = 30_000;
/// The shell's header: magic, version, op, flags, reserved, id, length.
const HDR: usize = 20;

pub fn deputy_escape() {
    let Some(port) = shell_port() else {
        note(b"[ATTACK-NOTE] deputy-escape: no desktop shell in this image\n");
        return;
    };
    const BODY: usize = HDR + 2 + APP.len() + PATH.len();
    let (mut tx, body) = ([0u8; BODY], BODY);
    tx[..4].copy_from_slice(&NDSH.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_OPEN_WITH.to_le_bytes());
    tx[12..16].copy_from_slice(&1u32.to_le_bytes());
    tx[16..20].copy_from_slice(&((body - HDR) as u32).to_le_bytes());
    tx[HDR..HDR + 2].copy_from_slice(&(APP.len() as u16).to_le_bytes());
    tx[HDR + 2..HDR + 2 + APP.len()].copy_from_slice(APP);
    tx[HDR + 2 + APP.len()..body].copy_from_slice(PATH);
    let mut rx = [0u8; HDR + 4];
    let (to, back) = (u64::from(port), rx.as_mut_ptr());
    let rc = mk_ipc_call_timeout(to, tx.as_ptr(), body, back, rx.len(), SHELL_MS);
    if rc < (HDR + 4) as i64 {
        verdict(b"deputy-escape", WHAT, rc.min(-1));
        return;
    }
    let status = i32::from_le_bytes([rx[HDR], rx[HDR + 1], rx[HDR + 2], rx[HDR + 3]]);
    if status < 0 {
        verdict(b"deputy-escape", WHAT, i64::from(status));
    } else {
        escaped(b"deputy-escape", WHAT, i64::from(status));
    }
}
