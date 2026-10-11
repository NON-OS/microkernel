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

//! What each refusal of the meter's accounting reads as on the wire.

use nonos_nox_meter::MeterError;

pub fn status_of(e: MeterError) -> i32 {
    match e {
        MeterError::NoChannel => -2,
        MeterError::Corrupt => -5,
        MeterError::Duplicate => -22,
        MeterError::Untrusted => -129,
        MeterError::Mismatch => -71,
        MeterError::NotMetered => -95,
        MeterError::BelowMinimum => -33,
        MeterError::Full => -23,
        // Settled above what this meter signed (E13); payment_proto names it.
        MeterError::Frozen => -1002,
        MeterError::OverDeposit => -28,
        MeterError::Expired => -62,
        MeterError::Overflow => -75,
        MeterError::OverBudget => -122,
    }
}
