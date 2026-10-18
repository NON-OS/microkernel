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

//! Settle drains at start, every six hours, and on op 10:
//! only the meter holds vouchers, and after expiry plus GRACE anyone may
//! close, so a voucher left on the device is the payee's risk.

use nonos_nox_config::{MAINNET, SEPOLIA};

use super::round::round;
use crate::settle::Settle;

pub const DRAIN_EVERY_MS: u64 = 6 * 60 * 60 * 1000;

/// One round on each chain; a chain with nothing owed, or no Settlement,
/// answers why and is left for the next round.
pub fn drain_all(s: &mut Settle) {
    for chain in [MAINNET, SEPOLIA] {
        let _ = round(s, chain);
    }
}
