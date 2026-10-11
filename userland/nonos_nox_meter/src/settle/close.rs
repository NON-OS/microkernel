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

//! CLOSE (plan/21, v1.4.7): close(channelId), once the channel is past its
//! expiry plus the contract's GRACE. Anyone may close: the meter's key is a
//! convenience there, never a need.

use nonos_nox_config::CLOSE_GAS;
use nonos_payment_proto::Close;
use nonos_settle_proto::Gas;

use super::channel_tx::tx;
use crate::error::MeterError;
use crate::fees::{covers, fees_ok};
use crate::open::TxRequest;
use crate::settle::calldata::close_data;
use crate::state::MeterState;

/// close, once `now` is past expiry plus `grace` (READ view 16, GRACE).
pub fn close_plan(
    s: &MeterState,
    c: &Close,
    settlement: &[u8; 20],
    g: &Gas,
    (now, grace): (u64, u64),
) -> Result<TxRequest, MeterError> {
    let ch = s.channels.iter().find(|x| x.id == c.channel_id && x.chain_id == c.chain_id);
    let open_until = ch.ok_or(MeterError::NoChannel)?.expiry.saturating_add(grace);
    if now < open_until {
        return Err(MeterError::TooEarly);
    }
    fees_ok(g.max_fee_per_gas, g.max_priority_fee_per_gas)?;
    covers(g.balance, CLOSE_GAS, g.max_fee_per_gas)?;
    Ok(tx(c.chain_id, settlement, g.tx_nonce, CLOSE_GAS, g, &close_data(&c.channel_id)))
}
