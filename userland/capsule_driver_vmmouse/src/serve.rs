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

//! The poll loop: the pointer's status looked at every few milliseconds,
//! whole packets read off its queue and posted. A status the host flags
//! as an error, or a queue that is no whole number of packets, is the
//! pointer lost: it is enabled again — the same ENABLE / STATUS / DATA
//! sequence as bring-up, which QEMU's hw/i386/vmmouse.c answers — and when
//! that fails it is left for a while before the next try.

use nonos_libc::mk_idle_ms;

use crate::backdoor::call;
use crate::constants::{
    CMD_ABSPOINTER_DATA, CMD_ABSPOINTER_STATUS, PACKETS_PER_LOOK, PACKET_WORDS, POLL_MS,
    QUEUE_MASK, RETRY_MS, STATUS_ERROR,
};
use crate::packet::{parse, Motion, Packet};
use crate::post::Pointer;
use crate::say;
use crate::setup::{disable, enable};

pub struct Driver {
    pub pointer: Pointer,
    /// The pointer was lost and has not come back.
    pub lost: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Look {
    /// Nothing queued.
    Idle,
    /// Packets were read; more may wait.
    Read,
    /// The pointer was lost and is not back.
    Lost,
}

pub fn run(d: &mut Driver) -> ! {
    loop {
        match step(d) {
            Look::Read => {}
            Look::Idle => {
                let _ = mk_idle_ms(POLL_MS);
            }
            Look::Lost => {
                let _ = mk_idle_ms(RETRY_MS);
            }
        }
    }
}

/// One look.
pub fn step(d: &mut Driver) -> Look {
    if d.lost {
        if enable().is_err() {
            return Look::Lost;
        }
        d.lost = false;
        say::back();
    }
    for _ in 0..PACKETS_PER_LOOK {
        let Ok(r) = call(CMD_ABSPOINTER_STATUS, 0) else { return lose(d) };
        let queued = r[0] & QUEUE_MASK;
        if r[0] & STATUS_ERROR == STATUS_ERROR || !queued.is_multiple_of(PACKET_WORDS) {
            return lose(d);
        }
        if queued == 0 {
            return Look::Idle;
        }
        let Ok(w) = call(CMD_ABSPOINTER_DATA, PACKET_WORDS) else { return lose(d) };
        d.pointer.post(parse(w));
    }
    Look::Read
}

fn lose(d: &mut Driver) -> Look {
    say::lost();
    disable();
    d.lost = true;
    // Buttons held when it went are let go: no packet will say so.
    d.pointer.post(Packet { buttons: 0, motion: Motion::Rel(0, 0), wheel: 0 });
    Look::Lost
}
