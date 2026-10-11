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
//! The landed reads (B7, v1.4.55, v1.4.56): for one address at a time, NOX
//! then ETH, each request spaced by a random few seconds, never while a
//! dispute or refund holds the call. The scan's address goes first while
//! one runs, then the due list's; nothing else is ever read.

use nonos_libc::{crypto_random, mk_uptime_ms};
use nonos_nox_config::{address, Contract};

use super::landed::spacing;
use super::landed_ask::{answered, ask, op_of};
use super::landed_held::{both, Reading};
use crate::wallet::order::send_step::SendStep;
use crate::wallet::screen::shield::consts::CHAIN_ID;
use crate::wallet::settle::LANDED;
use crate::wallet::state::State;

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
    if let Some(r) = l.reading.filter(|r| r.asked) {
        let Some(a) = settle.take(op_of(&r), LANDED) else { return false };
        let got = answered(op_of(&r), a);
        l.next_ms = now + spacing(draw());
        l.reading = r.nox.is_none().then_some(Reading { nox: Some(got), asked: false, ..r });
        match r.nox {
            Some(nox) if r.scan => l.scan.read(both(nox, got)),
            Some(nox) => l.due.read(r.address, both(nox, got), now),
            None => {}
        }
        return true;
    }
    let chain_id: u64 = CHAIN_ID.parse().unwrap_or(0);
    if sending || now < l.next_ms || address(chain_id, Contract::NoxToken).is_err() {
        return false;
    }
    let next = l.scan.next().map(|a| (a, true)).or_else(|| l.due.next(now).map(|a| (a, false)));
    let fresh = |(address, scan)| Reading { address, scan, nox: None, asked: false };
    let Some(r) = l.reading.or_else(|| next.map(fresh)) else { return false };
    let asked = ask(settle, &r, chain_id, now);
    if !asked {
        l.next_ms = now + spacing(draw());
    }
    l.reading = Some(Reading { asked, ..r });
    false
}
