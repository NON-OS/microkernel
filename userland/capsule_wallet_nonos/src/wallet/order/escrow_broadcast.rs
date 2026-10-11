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

//! A dispute's or a refund's broadcast (v1.4.53): settle's BROADCAST of
//! what the Shield signed, ASYNC; its hash, then orderOf until it lands.

use nonos_settle_proto::OP_BROADCAST;

use super::escrow_tx::GAS_EVERY_MS;
use super::escrowing::{Ctx, Escrowing};
use super::send_step::SendStep;
use crate::wallet::settle::{SettleCall, ESCROW};

const NOT_SENT: &str = "settle did not send the transaction; try again.";

/// BROADCAST the signed bytes; its hash, then orderOf until it lands.
pub fn broadcast(settle: &mut SettleCall, e: &mut Escrowing, c: &Ctx) -> bool {
    let SendStep::Broadcasting(raw) = &e.step else { return false };
    let Some(r) = settle.take(OP_BROADCAST, ESCROW) else {
        let body = [&c.chain_id.to_le_bytes()[..], raw].concat();
        let _ = settle.ask(OP_BROADCAST, ESCROW, &body, c.now_ms);
        return false;
    };
    e.step = match r.ok().and_then(|b| <[u8; 32]>::try_from(b.as_slice()).ok()) {
        Some(hash) => SendStep::Sent(hash),
        None => SendStep::Failed(NOT_SENT),
    };
    e.next_ms = c.now_ms + GAS_EVERY_MS;
    true
}
