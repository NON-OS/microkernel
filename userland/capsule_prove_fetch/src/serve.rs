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

//! fetch's endpoint: one frame, one answer. Between frames, a transfer
//! that is due is carried; a PUT left quiet for a minute is dropped.

use alloc::vec;

use nonos_libc::{mk_idle_ms, mk_ipc_recv_from, mk_ipc_reply, mk_uptime_ms};
use nonos_proof_abi::wire::fetch::{Stage, FETCH_MAGIC};
use nonos_proof_abi::wire::{reply, Header, FRAME_MAX};

use crate::callers::caller;
use crate::desk::{answer, Desk};

/// nonos.prove.fetch's service port.
const PORT: u64 = 5202;
const QUIET_MS: i64 = 60_000;

pub fn serve() -> ! {
    let mut desk = Desk::default();
    let mut buf = vec![0u8; FRAME_MAX];
    let (mut seen, mut since) = (None, mk_uptime_ms());
    loop {
        let stage = desk.t.as_ref().map(|t| t.stage);
        if matches!(stage, Some(Stage::Uploading | Stage::Downloading)) {
            crate::carry::run(&mut desk, &mut buf);
        }
        // A PUT whose bytes stopped coming, whoever else is asking meanwhile.
        let mark = desk.t.as_ref().map(|t| (t.stage, t.bytes.len()));
        if mark != seen {
            (seen, since) = (mark, mk_uptime_ms());
        } else if stage == Some(Stage::Receiving) && mk_uptime_ms() - since > QUIET_MS {
            desk.abandon();
        }
        if !take(&mut desk, &mut buf) {
            mk_idle_ms(50);
        }
    }
}

/// Answer what has come in, without waiting.
pub fn drain(desk: &mut Desk, buf: &mut [u8]) {
    while take(desk, buf) {}
}

/// One frame answered, if one had come.
fn take(desk: &mut Desk, buf: &mut [u8]) -> bool {
    let mut from = 0u32;
    let n = mk_ipc_recv_from(PORT, buf.as_mut_ptr(), buf.len(), 0, &mut from);
    let Some(n) = usize::try_from(n).ok().filter(|&n| n > 0 && from != 0) else {
        return false;
    };
    if let Some((h, body)) = Header::parse(&buf[..n], FETCH_MAGIC) {
        let (status, out) = answer(desk, caller(from), &h, body);
        let tx = reply(FETCH_MAGIC, &h, status, &out);
        let _ = mk_ipc_reply(from, tx.as_ptr(), tx.len());
    }
    true
}
