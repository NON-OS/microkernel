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
//! A job bound to RECEIPT: each frame from its caller as receipt_step.rs
//! decides. No ack, a wrong one, or a quiet spell, and it exits with the record
//! still sealed; a drop that finds none after a delivered receipt is said.

use alloc::vec;

use nonos_libc::{mk_debug, mk_exit, mk_ipc_recv_from, mk_ipc_reply, mk_seal_drop, mk_yield};
use nonos_proof_abi::wire::{reply, Header, ReceiptRequest, FRAME_MAX, PROVE_MAGIC};

use super::receipt::{make, Receipt};
use super::receipt_step::{step, Step, EINVAL};

const QUIET_MS: u64 = 5 * 60 * 1000;
const BROKEN: &[u8] = b"[PROVE-JOB] receipt: the record was gone at its ack: one consume broken\n";

pub fn serve(pid: u32, held: bool) -> ! {
    let mut buf = vec![0u8; FRAME_MAX];
    let mut made: Option<Receipt> = None;
    let mut last = now();
    loop {
        if now().saturating_sub(last) > QUIET_MS {
            mk_exit(0);
        }
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), buf.len(), 0, &mut sender);
        let Some(n) = usize::try_from(n).ok().filter(|&n| n > 0 && sender != 0) else {
            mk_yield();
            continue;
        };
        let Some((h, body)) = Header::parse(&buf[..n], PROVE_MAGIC) else { continue };
        last = now();
        let say = |status: i32, out: &[u8]| {
            let tx = reply(PROVE_MAGIC, &h, status, out);
            let _ = mk_ipc_reply(sender, tx.as_ptr(), tx.len());
        };
        match step(sender == pid, h.op, body, held, made.as_ref().map(|r| r.seal_no)) {
            Step::Refuse(e) => say(e, &[]),
            Step::Make => match ReceiptRequest::decode(body).ok_or(EINVAL).and_then(|r| make(&r)) {
                Ok(r) => {
                    say(0, &r.napr);
                    made = Some(r);
                }
                Err(e) => say(e, &[]),
            },
            Step::Drop => {
                let r = made.as_ref().expect("a drop follows a made receipt");
                let rc = mk_seal_drop(&r.slot, r.seal_no);
                if rc < 0 {
                    let _ = mk_debug(BROKEN.as_ptr(), BROKEN.len());
                }
                say(rc as i32, &[]);
                mk_exit(0);
            }
        }
    }
}

fn now() -> u64 {
    u64::try_from(nonos_libc::mk_time_millis()).unwrap_or(0)
}
