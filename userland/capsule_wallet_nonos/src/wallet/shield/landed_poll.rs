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

//! The landed rows' reads (B7, v1.4.52): the Shield's handed list once the
//! Shield screen opens, then each address's NOX, one READ balanceOf at a
//! time over settle ASYNC, never while a dispute or refund holds the call.

use nonos_libc::mk_uptime_ms;
use nonos_nox_config::{address, Contract};
use nonos_settle_proto::{parse_read_reply, Read, OP_READ};
use shield_wire::{fields, FRESH_HANDED, OP_FRESH_ADDRESS, STATUS_OK};

use super::client::call;
use super::landed::{balance_of, parse_handed, Landed};
use crate::wallet::screen::shield::consts::CHAIN_ID;
use crate::wallet::settle::LANDED;
use crate::wallet::state::{State, VIEW_SHIELD};

const BALANCE_OF: u16 = 1;

/// Whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    if state.view != VIEW_SHIELD || !super::open::here() {
        state.meter.landed = Landed::default();
        return false;
    }
    let sending = state.meter.purchase.as_deref().is_some_and(|p| p.escrow.is_some());
    let l = &mut state.meter.landed;
    let Some(list) = l.list.as_ref() else {
        let a = call(OP_FRESH_ADDRESS, &[FRESH_HANDED]).ok().filter(|a| a.status == STATUS_OK);
        l.list = Some(a.map(|a| parse_handed(fields(&a.body, "handed"))).unwrap_or_default());
        return false;
    };
    let Some((index, addr)) = list.get(l.next).copied() else { return false };
    let settle = &mut state.meter.settle;
    if sending {
        return false;
    }
    if let Some(r) = settle.take(OP_READ, LANDED) {
        l.next += 1;
        let nox = r.ok().and_then(|b| parse_read_reply(&b)).and_then(|w| balance_of(&w));
        if let Some(n) = nox.filter(|n| *n > 0) {
            l.found.push((index, n));
            return true;
        }
        return false;
    }
    let chain_id = CHAIN_ID.parse().unwrap_or(0);
    if address(chain_id, Contract::NoxToken).is_err() {
        return false;
    }
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(&addr);
    let body = Read { chain_id, view: BALANCE_OF, args: [w].into() }.encode();
    let now = mk_uptime_ms().max(0) as u64;
    let _ = body.map(|b| settle.ask(OP_READ, LANDED, &b, now));
    false
}
