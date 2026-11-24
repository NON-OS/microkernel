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

//! Asking a front for the family's grant, once a run: by box when the armed
//! box names a destination, else by the program's own policy. The answer is
//! kept, granted or not; a refusal is the kernel's facts, which a second ask
//! in the same family would not change.

use core::sync::atomic::{AtomicU8, Ordering::Relaxed};

use alloc::vec::Vec;

use linux_net_grant::wire::{
    ANON_OP_GRANT_BOX, ANON_OP_GRANT_POLICY, GRANTED, SOCKS_GRANT_BOX, SOCKS_GRANT_POLICY,
};
use nonos_libc::mk_ipc_call;

use super::kept::policy;

/// The front a stream goes through.
#[derive(Clone, Copy)]
pub enum Front {
    /// net.anon, for Anyone.
    Anyone,
    /// net.socks5, for the Nym mixnet.
    Nym,
}

const UNASKED: u8 = 0;
const YES: u8 = 1;
const NO: u8 = 2;

/// Each front's answer, by `Front` as its index.
static ANSWER: [AtomicU8; 2] = [const { AtomicU8::new(UNASKED) }; 2];

/// Whether the family holds a grant at `front`, whose endpoint is `port`,
/// asking it the first time.
pub fn ensure(front: Front, port: u32) -> bool {
    let slot = &ANSWER[front as usize];
    match slot.load(Relaxed) {
        YES => return true,
        NO => return false,
        _ => {}
    }
    let granted = ask(front, port);
    crate::linux::start_time::reached(crate::linux::start_time::GRANT);
    slot.store(if granted { YES } else { NO }, Relaxed);
    granted
}

fn ask(front: Front, port: u32) -> bool {
    let by_box = crate::linux::boxed::armed_dests();
    let body: Vec<u8> = match (by_box, policy()) {
        (true, _) => Vec::new(),
        (false, Some(p)) => p.to_vec(),
        (false, None) => return false,
    };
    match front {
        Front::Anyone => {
            let op = if by_box { ANON_OP_GRANT_BOX } else { ANON_OP_GRANT_POLICY };
            let reply = super::super::anon_call::call(port, op, &body, 1);
            matches!(reply, Some((0, b)) if b.first() == Some(&GRANTED))
        }
        Front::Nym => {
            let mark = if by_box { SOCKS_GRANT_BOX } else { SOCKS_GRANT_POLICY };
            let tx = [&[mark][..], &body].concat();
            let mut rx = [0u8; 2];
            let n = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
            n == 1 && rx[0] == GRANTED
        }
    }
}
