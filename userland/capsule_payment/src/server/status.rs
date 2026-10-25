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

//! What each refusal of the meter's accounting reads as on the wire
//! (payment_proto::status).

use nonos_nox_meter::MeterError;
use nonos_payment_proto as p;

pub fn status_of(e: MeterError) -> i32 {
    match e {
        MeterError::NoChannel => p::ENOENT,
        MeterError::Corrupt => p::EIO,
        MeterError::Duplicate => p::EINVAL,
        MeterError::Untrusted => p::EKEYREJECTED,
        MeterError::Mismatch => p::EPROTO,
        MeterError::NotMetered => p::EOPNOTSUPP,
        MeterError::BelowMinimum => p::EDOM,
        MeterError::Full => p::ENFILE,
        MeterError::Frozen => p::EFROZEN,
        MeterError::NoGas => p::EGAS,
        MeterError::OverDeposit => p::ENOSPC,
        MeterError::Expired => p::ETIME,
        MeterError::Overflow => p::EOVERFLOW,
        MeterError::OverBudget => p::EDQUOT,
        MeterError::NothingOwed => p::ENOENT,
        MeterError::TooEarly => p::ETIME,
        MeterError::Paused => p::EBUSY,
    }
}
