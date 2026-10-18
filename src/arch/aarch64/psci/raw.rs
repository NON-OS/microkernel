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

use super::error::PsciError;
use super::state::method;
use crate::arch::aarch64::smccc::call;

/// A PSCI call through `smccc::call`, which declares every register a
/// firmware may change. PSCI 0.2 firmware speaks SMCCC 1.0, so X1-X17 may
/// come back changed, not only X0.
pub fn psci_call(func: u32, arg0: u64, arg1: u64, arg2: u64) -> i64 {
    match method() {
        Some(via) => call(via, func, [arg0, arg1, arg2]),
        // No firmware to ask: answered as PSCI answers a call it lacks.
        None => i64::from(PsciError::NotSupported.code()),
    }
}

pub fn psci_call0(func: u32) -> i64 {
    psci_call(func, 0, 0, 0)
}

pub fn psci_call1(func: u32, arg0: u64) -> i64 {
    psci_call(func, arg0, 0, 0)
}

pub fn psci_call2(func: u32, arg0: u64, arg1: u64) -> i64 {
    psci_call(func, arg0, arg1, 0)
}
