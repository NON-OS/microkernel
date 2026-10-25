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

//! What a job does once its reads are in: GAS for the jobs that send, or the
//! order's answer for ORDER_NEW and ORDER_STATUS (v1.4.20, status.rs).

use alloc::vec::Vec;

use nonos_payment_proto::OrderMade;

use super::ask::{Ask, Next, Step};
use super::status::status;
use super::words::{first_u128, word};
use super::work::Work;
use crate::error::MeterError;
use crate::orders::Order;

pub(super) type Got = [Vec<[u8; 32]>];

pub(super) fn after_reads(work: &Work, got: &Got) -> Step {
    match work {
        Work::Claim { .. } if first_u128(got.first()) == Some(0) => Err(MeterError::NothingOwed),
        Work::OrderNew(o) => made(o, got),
        Work::OrderStatus(o) => status(o, got),
        _ => Ok(Next::Ask(Ask::Gas)),
    }
}

/// paused() strictly false, when read; depositAddress the order's own.
fn made(o: &Order, got: &Got) -> Step {
    let (deposit, paused) = got.split_last().ok_or(MeterError::Mismatch)?;
    if let Some(p) = paused.first() {
        match first_u128(Some(p)) {
            Some(0) => {}
            Some(1) => return Err(MeterError::Paused),
            _ => return Err(MeterError::Mismatch),
        }
    }
    if deposit.as_slice() != [word(&o.deposit)] {
        return Err(MeterError::Untrusted);
    }
    let m = OrderMade { order_id: o.order_id, deposit: o.deposit, terms: o.terms };
    Ok(Next::Keep(alloc::boxed::Box::new(*o), m.encode()))
}
