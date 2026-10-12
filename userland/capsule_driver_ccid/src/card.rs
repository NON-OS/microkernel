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
//! A command sent to a slot and the card's answer read back: its data and
//! bmICCStatus, or the reader's bError when it failed it. Each answer is
//! waited on for WAIT_MS, again after each time extension.

use alloc::vec::Vec;

use nonos_libc::{mk_idle_ms, Deadline};

use crate::ccid::{answers, command, exchange, reply, Outcome, HEADER};
use crate::device::Dev;
use crate::wire::{E_INVAL, E_IO, E_NODEV};

/// How long a reader is waited on with no word from it.
const WAIT_MS: u64 = 3_000;
/// The look between polls; a card's answer takes milliseconds at best.
const IDLE_MS: u64 = 1;

/// Why a command came to nothing: an errno, and `[bError, bmICCStatus]`
/// when the reader said.
pub type Fault = (i32, Option<[u8; 2]>);

pub fn ask(d: &mut Dev, kind: u8, slot: u8, data: &[u8]) -> Result<(Vec<u8>, u8), Fault> {
    let Some(r) = d.port().map(|p| p.r) else { return Err((E_NODEV, None)) };
    if slot > r.max_slot || HEADER + data.len() > r.max_message {
        return Err((E_INVAL, None));
    }
    let Some(mut bus) = d.bus() else { return Err((E_NODEV, None)) };
    let cmd = command(kind, slot, d.next_seq(), data);
    let mut deadline = Deadline::after_ms(WAIT_MS);
    let wait = |fresh: bool| {
        if fresh {
            deadline = Deadline::after_ms(WAIT_MS);
        }
        let _ = mk_idle_ms(IDLE_MS);
        !deadline.expired()
    };
    let msg = exchange(&mut bus, &cmd, r.max_message, wait).map_err(|e| (e, None))?;
    let Some(a) = reply(&msg).filter(|a| a.kind == answers(kind)) else { return Err((E_IO, None)) };
    match a.outcome {
        Outcome::Done => Ok((a.data.to_vec(), a.icc)),
        Outcome::Failed(why) => Err((E_IO, Some([why, a.icc]))),
        Outcome::MoreTime => Err((E_IO, None)),
    }
}
