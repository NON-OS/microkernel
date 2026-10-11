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
//! order's answer for ORDER_NEW and ORDER_STATUS (v1.4.20).

use alloc::vec::Vec;

use nonos_payment_proto::{OrderMade, OrderStatus};

use super::ask::{Ask, Next, Step};
use super::words::{first_u128, u128_of, word};
use super::work::Work;
use crate::error::MeterError;
use crate::orders::Order;

type Got = [Vec<[u8; 32]>];

pub(super) fn after_reads(work: &Work, got: &Got) -> Step {
    match work {
        Work::Claim { .. } if first_u128(got.first()) == Some(0) => Err(MeterError::NothingOwed),
        Work::OrderNew(o) => made(o, got),
        Work::OrderStatus(_) => status(got).map(Next::Done),
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

/// orderOf's state and funded, and the NOX waiting for the order: the
/// deposit address's balance plus what the pool credits it.
fn status(got: &Got) -> Result<Vec<u8>, MeterError> {
    let of = got.first().filter(|w| w.len() == 4).ok_or(MeterError::Mismatch)?;
    let state = u128_of(&of[0]).and_then(|s| u8::try_from(s).ok());
    let funded = u128_of(&of[1]);
    let balance = first_u128(got.get(1));
    let credit = got.get(2).map_or(Some(0), |r| first_u128(Some(r)));
    let (Some(state), Some(funded), Some(balance), Some(credit)) = (state, funded, balance, credit)
    else {
        return Err(MeterError::Mismatch);
    };
    let pending = balance.checked_add(credit).ok_or(MeterError::Overflow)?;
    Ok(OrderStatus { state, funded, pending }.encode())
}
