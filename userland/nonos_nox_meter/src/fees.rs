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

//! The fee caps every transaction the meter signs stays under: the chain sets the base fee, so a lying settle can overpay only
//! the tip, at most MAX_PRIORITY_FEE_PER_GAS a gas.

use nonos_nox_config::{MAX_FEE_PER_GAS, MAX_PRIORITY_FEE_PER_GAS};

use crate::error::MeterError;

pub fn fees_ok(max_fee: u128, tip: u128) -> Result<(), MeterError> {
    let ok = max_fee <= MAX_FEE_PER_GAS && tip <= MAX_PRIORITY_FEE_PER_GAS && tip <= max_fee;
    ok.then_some(()).ok_or(MeterError::NoGas)
}

/// Whether `balance` pays `gas` at `max_fee` a gas.
pub fn covers(balance: u128, gas: u64, max_fee: u128) -> Result<(), MeterError> {
    let need = (gas as u128).checked_mul(max_fee);
    need.filter(|n| balance >= *n).map(|_| ()).ok_or(MeterError::NoGas)
}
