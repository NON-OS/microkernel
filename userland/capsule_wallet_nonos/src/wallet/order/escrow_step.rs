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

//! One step of a dispute or a refund through settle (v1.4.53): the one
//! settle request its step needs, or its answer taken. The Shield's
//! signature is the caller's (escrow_sign).

use super::escrow_broadcast::broadcast;
use super::escrow_read::read;
use super::escrow_refund::refund_to;
use super::escrow_tx::{gas, quoting};
use super::escrowing::{Ctx, Escrowing};
use super::send_step::SendStep;
use crate::wallet::settle::SettleCall;

/// Whether the step moved.
pub fn step(settle: &mut SettleCall, e: &mut Escrowing, c: &Ctx) -> bool {
    match &e.step {
        SendStep::Reading | SendStep::Sent(_) => read(settle, e, c),
        SendStep::RefundTo(of) => refund_to(e, c, *of),
        SendStep::Quoting(_) => quoting(settle, e, c),
        SendStep::Gas { .. } => gas(settle, e, c),
        SendStep::Broadcasting(_) => broadcast(settle, e, c),
        _ => false,
    }
}
