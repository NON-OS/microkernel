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

//! Where PAY's charge lands, decided once for PAY (capsule_payment's
//! handlers/pay_charge.rs) and FUNDED (funded.rs) alike.

use nonos_payment_proto::PayRequest;

use crate::channel::Channel;
use crate::error::MeterError;
use crate::state::MeterState;

/// The channel `r`'s charge of `amount` lands on at `now`, as charged with
/// no usage root yet, or why PAY refuses it: MeterState::charged's choice
/// of a live channel for r's payee, listing and chain, that channel's
/// freeze, margin and deposit, and the owner's budget, global and for the
/// listing; then the unit, since a channel's vouchers carry the unit it
/// was opened for and no other.
pub fn landing(
    s: &MeterState,
    r: &PayRequest<'_>,
    amount: u128,
    now: u64,
) -> Result<Channel, MeterError> {
    let (_, at) = s.charged(&r.payee, &r.listing_id, r.chain_id, amount, [0; 32], now)?;
    if at.unit != r.unit {
        return Err(MeterError::Mismatch);
    }
    Ok(at)
}
