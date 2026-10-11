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

//! The landed reads (B7, v1.4.56): one address per settle READ of NOX's
//! balanceOf, spaced by a random few seconds, never while a dispute or
//! refund holds the call. The scan's address goes first while one runs,
//! then the due list's; nothing else is ever read.

use nonos_libc::{crypto_random, mk_uptime_ms};
use nonos_nox_config::{address, Contract};
use nonos_settle_proto::{parse_read_reply, Read, OP_READ};

use super::landed::{balance_of, spacing};
use crate::wallet::order::send_step::SendStep;
use crate::wallet::screen::shield::consts::CHAIN_ID;
use crate::wallet::settle::LANDED;
use crate::wallet::state::State;

const BALANCE_OF: u16 = 1;

fn draw() -> u16 {
    let mut b = [0u8; 2];
    let _ = crypto_random(b.as_mut_ptr(), b.len());
    u16::from_le_bytes(b)
}

/// Whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    super::due_feed::feed(state);
    let now = mk_uptime_ms().max(0) as u64;
    let escrow = state.meter.purchase.as_deref().and_then(|p| p.escrow.as_ref());
    let sending = escrow.is_some_and(|e| !matches!(e.step, SendStep::Done | SendStep::Failed(_)));
    let (l, settle) = (&mut state.meter.landed, &mut state.meter.settle);
    if let Some((addr, scan)) = l.reading {
        let Some(r) = settle.take(OP_READ, LANDED) else { return false };
        let nox = r.ok().and_then(|b| parse_read_reply(&b)).and_then(|w| balance_of(&w));
        match scan {
            true => l.scan.read(nox),
            false => l.due.read(addr, nox, now),
        }
        (l.reading, l.next_ms) = (None, now + spacing(draw()));
        return true;
    }
    let chain_id: u64 = CHAIN_ID.parse().unwrap_or(0);
    if sending || now < l.next_ms || address(chain_id, Contract::NoxToken).is_err() {
        return false;
    }
    let next = l.scan.next().map(|a| (a, true)).or_else(|| l.due.next(now).map(|a| (a, false)));
    let Some((addr, scan)) = next else { return false };
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(&addr);
    let body = Read { chain_id, view: BALANCE_OF, args: [w].into() }.encode();
    if body.is_some_and(|b| settle.ask(OP_READ, LANDED, &b, now).is_ok()) {
        l.reading = Some((addr, scan));
    }
    false
}
